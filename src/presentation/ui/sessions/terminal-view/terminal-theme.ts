import { COLORS, FONT_SIZE_SCALE } from '@faber-ui/react';
import type { ITheme } from '@xterm/xterm';

const CSS_VARIABLE_NAME = /var\((--[^,)]+)/;

/**
 * xterm.js paints on a canvas and needs real colors, not CSS variables, so each faber-ui
 * token is read from the stylesheet the app already loaded.
 */
function resolveColor(token: string): string {
  const variableName = CSS_VARIABLE_NAME.exec(token)?.[1];
  if (!variableName) {
    return token;
  }
  return getComputedStyle(document.documentElement).getPropertyValue(variableName).trim();
}

export function createTerminalTheme(): ITheme {
  return {
    background: resolveColor(COLORS.BACKGROUND_PRIMARY),
    foreground: resolveColor(COLORS.TEXT_PRIMARY),
    cursor: resolveColor(COLORS.PRIMARY),
    cursorAccent: resolveColor(COLORS.BACKGROUND_PRIMARY),
    selectionBackground: resolveColor(COLORS.BORDER_STRONG),
  };
}

export const TERMINAL_FONT_SIZE = Number.parseInt(FONT_SIZE_SCALE.SM, 10);
