// Copy this spec into each experimental checkout's e2e/tests directory after implementation.
import { expect, test, type Page } from '../helpers/fixtures';
import { agpjOk, mainProject, mainUrl, resetBoard } from '../helpers/agpj';
import { resetPreferences } from '../helpers/preferences';
import { openHeaderItem } from '../helpers/ui';

const time = (offsetHours: number) => {
  const now = new Date(Date.now() + offsetHours * 3_600_000);
  return now.toISOString().slice(11, 16);
};

const sleepStatus = (page: Page) =>
  page.getByRole('banner').getByText(/sleep|asleep/i);

for (const [name, viewport] of [
  ['desktop', { width: 1280, height: 900 }],
  ['phone width', { width: 390, height: 844 }],
] as const) {
  test(`sleep schedule on ${name}`, async ({ page }, info) => {
    resetBoard(mainProject);
    await resetPreferences();
    agpjOk(mainProject, ['mode', 'schedule', '--off']);
    agpjOk(mainProject, ['mode', 'auto']);
    await page.setViewportSize(viewport);
    await page.goto(mainUrl());
    await expect(sleepStatus(page)).toHaveCount(0);

    // Move the window around the present via the real CLI and observe the open board.
    const start = time(-1);
    const end = time(1);
    agpjOk(mainProject, ['mode', 'schedule', '--start', start, '--end', end]);
    await expect(sleepStatus(page)).toBeVisible({ timeout: 15_000 });

    await openHeaderItem(page, 'Settings');
    const dialog = page.getByRole('dialog', { name: 'Settings' });
    await expect(dialog).toBeVisible();
    await expect.soft(dialog.getByRole('radiogroup', { name: 'Mode' })).toHaveCount(0, { timeout: 500 });
    const inputs = dialog.locator('input[type="time"]');
    await expect(inputs).toHaveCount(2);
    await expect(inputs.first()).toHaveValue(start);
    await expect(inputs.last()).toHaveValue(end);
    const height = await inputs.first().evaluate((input) => {
      let block = input.parentElement;
      while (block && block.querySelectorAll('input[type="time"]').length < 2) {
        block = block.parentElement;
      }
      if (!block) return null;
      const section = block.closest('section') ?? block;
      return {
        controls: Math.round(block.getBoundingClientRect().height),
        section: Math.round(section.getBoundingClientRect().height),
      };
    });
    console.log(`AFK #759 ${name} schedule layout px: ${JSON.stringify(height)}`);
    await page.screenshot({ path: info.outputPath(`${name}-schedule.png`) });

    // Change one input in the UI, read it from the running backend, and reopen the UI.
    const revisedStart = time(-2);
    await inputs.first().fill(revisedStart);
    await inputs.first().press('Tab');
    const save = dialog.getByRole('button', { name: /^save/i });
    if (await save.count()) await save.first().click();
    await expect.poll(() => agpjOk(mainProject, ['mode', 'schedule']).stdout).toContain(revisedStart);
    await page.reload();
    await expect(sleepStatus(page)).toBeVisible();
    await openHeaderItem(page, 'Settings');
    await expect(page.getByRole('dialog', { name: 'Settings' }).locator('input[type="time"]').first())
      .toHaveValue(revisedStart);

    agpjOk(mainProject, ['mode', 'schedule', '--start', time(2), '--end', time(4)]);
    await expect(sleepStatus(page)).toHaveCount(0, { timeout: 15_000 });
  });
}
