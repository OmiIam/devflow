import { test, expect } from '@playwright/test';
import { FocusPage } from './pages/focusPage';

test.describe('Focus journey', () => {
  test('user can start, pause, and resume a focus session', async ({ page }) => {
    const focusPage = new FocusPage(page);
    await focusPage.goto();

    await focusPage.startFocusSession();
    await expect(focusPage.status()).toContainText(/in progress/i);

    await focusPage.pauseFocusSession();
    await expect(focusPage.status()).toContainText(/paused/i);

    await focusPage.resumeFocusSession();
    await expect(focusPage.status()).toContainText(/in progress/i);
  });
});
