import { Page, expect } from '@playwright/test';

export class FocusPage {
  constructor(private readonly page: Page) {}

  async goto() {
    await this.page.goto('/');
    await expect(this.page.getByRole('heading', { name: /devflow/i })).toBeVisible();
  }

  async startFocusSession() {
    await this.page.getByRole('button', { name: /start focus/i }).click();
  }

  async pauseFocusSession() {
    await this.page.getByRole('button', { name: /pause/i }).click();
  }

  async resumeFocusSession() {
    await this.page.getByRole('button', { name: /resume/i }).click();
  }

  status() {
    return this.page.getByTestId('focus-status');
  }
}
