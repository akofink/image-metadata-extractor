import { test as base, Page } from '@playwright/test';

/**
 * Extended test fixture that blocks analytics and ad requests during E2E tests.
 * This prevents test data from polluting production analytics.
 */
export const test = base.extend<{ page: Page }>({
  page: async ({ page }, use) => {
    // Block Google Analytics, Google Tag Manager, and AdSense
    await page.route('**/*', (route) => {
      const url = route.request().url();

      // List of analytics/ads domains to block
      const blockedDomains = [
        'google-analytics.com',
        'googletagmanager.com',
        'googlesyndication.com',
        'doubleclick.net',
        'analytics.google.com',
        'googleadservices.com',
      ];

      // Block if URL matches any blocked domain
      if (blockedDomains.some(domain => url.includes(domain))) {
        route.abort();
      } else {
        route.continue();
      }
    });

    await use(page);
  },
});

export { expect } from '@playwright/test';
