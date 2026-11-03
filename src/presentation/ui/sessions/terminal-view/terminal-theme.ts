import { COLORS, FONT_SIZE_SCALE } from '@faber-ui/react';
import type { ITheme } from '@xterm/xterm';

import { BRAND_FONT_FAMILIES } from '../../shared/brand-tokens';

const CSS_VARIABLE_NAME = /var\((--[^,)]+)/;

/**
 * xterm.js paints on a canvas and needs real values, not CSS variables, so each token is
 * read from the stylesheet the app already loaded.
 */
function resolveToken(token: string): string {
  const variableName = CSS_VARIABLE_NAME.exec(token)?.[1];
  if (!variableName) {
    return token;
  }
  return getComputedStyle(document.documentElement).getPropertyValue(variableName).trim();
}

export function createTerminalTheme(): ITheme {
  return {
    background: resolveToken(COLORS.BACKGROUND_PRIMARY),
    foreground: resolveToken(COLORS.TEXT_PRIMARY),
    cursor: resolveToken(COLORS.PRIMARY),
    cursorAccent: resolveToken(COLORS.BACKGROUND_PRIMARY),
    selectionBackground: resolveToken(COLORS.BORDER_STRONG),
  };
}

export const TERMINAL_FONT_SIZE = Number.parseInt(FONT_SIZE_SCALE.SM, 10);

export function resolveTerminalFontFamily(): string {
  return resolveToken(BRAND_FONT_FAMILIES.MONO);
}
