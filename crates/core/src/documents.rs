use std::{
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use anyhow::{Context, Result, bail, ensure};
use sha2::{Digest, Sha256};

use crate::{Document, Store, check_path};

const MAX_DOCUMENT_BYTES: u64 = 1024 * 1024;
const MAX_CRITERIA: usize = 64;

pub(crate) const REQUEST_TEMPLATE: &str = "# \n";
pub(crate) const WORK_TEMPLATE: &str = "# \n\n## Acceptance\n\n- \n\n## Result\n";
pub(crate) const TASK_TEMPLATE: &str = "# \n<!-- afk:execution -->\n\n## Execution agents\n\nNo execution recorded.\n\n<!-- /afk:execution -->\n\n## Acceptance\n\n- \n\n## Result\n";
pub(crate) const QUESTION_TEMPLATE: &str = "# \n\n## Answer\n";
const EXECUTION_START: &str = "<!-- afk:execution -->\n";
const EXECUTION_END: &str = "<!-- /afk:execution -->\n";

pub(crate) struct Spec {
    pub title: String,
    pub digest: String,
    pub final_hash: Option<String>,
    pub criteria: u32,
}

impl Store {
    pub(crate) fn require_unowned_path_available(&self, doc: &Document) -> Result<()> {
        match fs::symlink_metadata(&doc.path) {
            Ok(_) => bail!(
                "unreferenced file {}; preserve it elsewhere before reserving this path",
                doc.path.display()
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    pub(crate) fn prepare_file(&self, doc: &Document, template: &str) -> Result<()> {
        check_path(&self.base, true)?;
        check_path(
            doc.path.parent().context("document directory missing")?,
            true,
        )?;
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600).custom_flags(libc::O_NOFOLLOW);
        }
        match options.open(&doc.path) {
            Ok(mut file) => file
                .write_all(template.as_bytes())
                .with_context(|| format!("write template to {}", doc.path.display())),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                check_path(&doc.path, false)
            }
            Err(error) => Err(error).with_context(|| format!("prepare {}", doc.path.display())),
        }
    }

    pub(crate) fn read_request(&self, id: i64) -> Result<String> {
        let path = self.document("requests", id).path;
        let text = self.read_file(&path)?;
        let has_content = text
            .lines()
            .any(|line| !crate::plain_title::strip_inline_markdown(line).is_empty());
        ensure!(has_content, "write the request to {}", path.display());
        Ok(hash(&text))
    }

    pub(crate) fn read_question(&self, id: i64, answered: bool) -> Result<Spec> {
        let doc = self.document("questions", id);
        self.read_spec(&doc, "Answer", false, answered, false)
    }

    pub(crate) fn read_work(&self, doc: &Document, finished: bool) -> Result<Spec> {
        let task = doc.path.parent() == Some(self.base.join("tasks").as_path());
        self.read_spec(doc, "Result", true, finished, task)
    }

    pub(crate) fn render_task_runs(&self, doc: &Document, runs: &str) -> Result<()> {
        let original = self.read_file(&doc.path)?;
        let (first, body) = task_parts(&original)?;
        let rendered = format!(
            "{first}{EXECUTION_START}\n## Execution agents\n\n{runs}\n{EXECUTION_END}{body}"
        );
        ensure!(
            rendered.len() as u64 <= MAX_DOCUMENT_BYTES,
            "AFK document exceeds 1 MiB"
        );
        if rendered != original {
            let parent = doc.path.parent().context("document directory missing")?;
            let mut temp = tempfile::NamedTempFile::new_in(parent)?;
            temp.write_all(rendered.as_bytes())?;
            temp.as_file().sync_all()?;
            ensure!(
                self.read_file(&doc.path)? == original,
                "task was edited while updating its execution history; retry"
            );
            temp.persist(&doc.path)?;
        }
        Ok(())
    }

    fn read_file(&self, path: &Path) -> Result<String> {
        check_path(&self.base, true)?;
        check_path(path.parent().context("document directory missing")?, true)?;
        check_path(path, false).with_context(|| format!("restore {}", path.display()))?;
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
        }
        let file = options.open(path)?;
        ensure!(
            file.metadata()?.is_file(),
            "AFK documents must be regular files"
        );
        let mut text = String::new();
        file.take(MAX_DOCUMENT_BYTES + 1)
            .read_to_string(&mut text)
            .with_context(|| format!("read UTF-8 Markdown from {}", path.display()))?;
        ensure!(
            text.len() as u64 <= MAX_DOCUMENT_BYTES,
            "AFK document exceeds 1 MiB"
        );
        Ok(text)
    }

    fn read_spec(
        &self,
        doc: &Document,
        final_heading: &str,
        acceptance: bool,
        finished: bool,
        task: bool,
    ) -> Result<Spec> {
        let text = self.read_file(&doc.path)?;
        let text = if task {
            let (first, body) = task_parts(&text)?;
            format!("{first}{body}")
        } else {
            text
        };
        let (spec, result) = split_section(&text, final_heading)
            .with_context(|| format!("check {}", doc.path.display()))?;
        let title: String = crate::plain_title::first_line_plain(spec)
            .chars()
            .filter(|c| !c.is_control())
            .take(100)
            .collect();
        ensure!(
            !title.trim().is_empty(),
            "add a title to {}",
            doc.path.display()
        );
        let final_hash = result.filter(|body| !body.trim().is_empty()).map(hash);
        ensure!(
            !finished || final_hash.is_some(),
            "write a nonempty ## {final_heading} in {}",
            doc.path.display()
        );
        let criteria = if acceptance {
            let (_, criteria) = split_section(spec, "Acceptance")?;
            let list = criteria.context("add an ## Acceptance section")?;
            let count = list
                .lines()
                .take_while(|line| !line.starts_with("## "))
                .filter(|line| {
                    line.strip_prefix("- ")
                        .is_some_and(|item| !item.trim().is_empty())
                })
                .count();
            ensure!(
                (1..=MAX_CRITERIA).contains(&count),
                "write 1–64 acceptance bullets in {}",
                doc.path.display()
            );
            count as u32
        } else {
            0
        };
        Ok(Spec {
            title,
            digest: hash(spec),
            final_hash,
            criteria,
        })
    }
}

fn task_parts(text: &str) -> Result<(&str, &str)> {
    let (first, body) = text.split_at(text.find('\n').map_or(text.len(), |n| n + 1));
    let body = if let Some(generated) = body.strip_prefix(EXECUTION_START) {
        generated
            .split_once(EXECUTION_END)
            .context("restore the closing AFK execution marker")?
            .1
    } else {
        body
    };
    Ok((first, body))
}

fn split_section<'a>(text: &'a str, heading: &str) -> Result<(&'a str, Option<&'a str>)> {
    let mut offset = 0;
    let mut section = None;
    let marker = format!("## {heading}");
    let mut fence: Option<(u8, usize)> = None;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim();
        let first = trimmed.as_bytes().first().copied();
        if let Some(character @ (b'`' | b'~')) = first {
            let length = trimmed
                .bytes()
                .take_while(|&byte| byte == character)
                .count();
            match fence {
                Some((open, minimum))
                    if open == character && length >= minimum && trimmed.len() == length =>
                {
                    fence = None
                }
                None if length >= 3 => fence = Some((character, length)),
                _ => {}
            }
        }
        if fence.is_none() && line.trim_end() == marker {
            if section.is_some() {
                bail!("duplicate {marker} heading");
            }
            section = Some((offset, offset + line.len()));
        }
        offset += line.len();
    }
    Ok(match section {
        Some((start, body)) => (&text[..start], Some(&text[body..])),
        None => (text, None),
    })
}

fn hash(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.trim_end().as_bytes()))
}
