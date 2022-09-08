import { expect, test } from '@playwright/test';
import { Page } from 'playwright';

import { colors } from '../src/config.json';
import { ILocation, ITunnelEndpoint, TunnelState } from '../src/shared/daemon-rpc-types';
import {
  getBackgroundColor,
  getColor,
  mockIpcHandle,
  sendMockIpcResponse,
  startApp,
} from './utils';

const UNSECURED_COLOR = colors.red;
const SECURE_COLOR = colors.green;
const WHITE_COLOR = colors.white;

const mockLocation: ILocation = {
  country: 'Sweden',
  city: 'Gothenburg',
  latitude: 58,
  longitude: 12,
  mullvadExitIp: false,
};

const getLabel = () => appWindow.locator('span[role="status"]');
const getHeader = () => appWindow.locator('header');
const getScreenShotPath = (test: string) => `e2e/screenshots/tunnel-state/${test}.png`;

let appWindow: Page;

test.beforeAll(async () => {
  const startAppResponse = await startApp();
  appWindow = startAppResponse.appWindow;
});

test.afterAll(async () => {
  await appWindow.close();
});

/**
 * Disconnected state
 */
test('App should show disconnected tunnel state', async () => {
  await mockIpcHandle<ILocation>({
    channel: 'location-get',
    response: mockLocation,
  });

  await sendMockIpcResponse<TunnelState>({
    channel: 'tunnel-',
    response: { state: 'disconnected' },
  });

  const statusLabel = getLabel();
  await expect(statusLabel).toContainText(/unsecured connection/i);
  const labelColor = await getColor(statusLabel);
  expect(labelColor).toBe(UNSECURED_COLOR);

  const header = getHeader();
  const headerColor = await getBackgroundColor(header);
  expect(headerColor).toBe(UNSECURED_COLOR);

  await appWindow.screenshot({ path: getScreenShotPath('unsecured') });
});

/**
 * Connecting state
 */
test('App should show connecting tunnel state', async () => {
  await mockIpcHandle<ILocation>({
    channel: 'location-get',
    response: mockLocation,
  });

  await sendMockIpcResponse<TunnelState>({
    channel: 'tunnel-',
    response: { state: 'connecting' },
  });

  const statusLabel = getLabel();
  await expect(statusLabel).toContainText(/creating secure connection/i);
  const labelColor = await getColor(statusLabel);
  expect(labelColor).toBe(WHITE_COLOR);

  const header = getHeader();
  const headerColor = await getBackgroundColor(header);
  expect(headerColor).toBe(SECURE_COLOR);

  await appWindow.screenshot({ path: getScreenShotPath('connecting') });
});

/**
 * Connected state
 */
test('App should show connected tunnel state', async () => {
  const location: ILocation = { ...mockLocation, mullvadExitIp: true };
  await mockIpcHandle<ILocation>({
    channel: 'location-get',
    response: location,
  });

  const endpoint: ITunnelEndpoint = {
    address: 'wg10:80',
    protocol: 'tcp',
    quantumResistant: false,
    tunnelType: 'wireguard',
  };
  await sendMockIpcResponse<TunnelState>({
    channel: 'tunnel-',
    response: { state: 'connected', details: { endpoint, location } },
  });

  const statusLabel = getLabel();
  await expect(statusLabel).toContainText(/secure connection/i);
  const labelColor = await getColor(statusLabel);
  expect(labelColor).toBe(SECURE_COLOR);

  const header = getHeader();
  const headerColor = await getBackgroundColor(header);
  expect(headerColor).toBe(SECURE_COLOR);

  await appWindow.screenshot({ path: getScreenShotPath('secure') });
});

/**
 * Disconnecting state
 */
test('App should show disconnecting tunnel state', async () => {
  await mockIpcHandle<ILocation>({
    channel: 'location-get',
    response: mockLocation,
  });

  await sendMockIpcResponse<TunnelState>({
    channel: 'tunnel-',
    response: { state: 'disconnecting', details: 'nothing' },
  });

  const statusLabel = getLabel();
  await expect(statusLabel).toBeEmpty();
  const labelColor = await getColor(statusLabel);
  expect(labelColor).toBe(WHITE_COLOR);

  const header = getHeader();
  const headerColor = await getBackgroundColor(header);
  expect(headerColor).toBe(UNSECURED_COLOR);

  await appWindow.screenshot({ path: getScreenShotPath('disconnecting') });
});

/**
 * Error state
 */
test('App should show error tunnel state', async () => {
  await mockIpcHandle<ILocation>({
    channel: 'location-get',
    response: mockLocation,
  });

  await sendMockIpcResponse<TunnelState>({
    channel: 'tunnel-',
    response: { state: 'error', details: { cause: { reason: 'is_offline' } } },
  });

  const statusLabel = getLabel();
  await expect(statusLabel).toContainText(/blocked connection/i);
  const labelColor = await getColor(statusLabel);
  expect(labelColor).toBe(WHITE_COLOR);

  const header = getHeader();
  const headerColor = await getBackgroundColor(header);
  expect(headerColor).toBe(SECURE_COLOR);

  await appWindow.screenshot({ path: getScreenShotPath('error') });
});
