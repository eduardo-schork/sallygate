export type TOutputListener = (bytes: Uint8Array) => void;

/**
 * Holds terminal output until a terminal is listening. A shell starts writing (the login
 * banner, the prompt) before the terminal on screen has mounted, and none of it may be lost.
 */
export type TSessionOutputBuffer = {
  readonly push: (bytes: Uint8Array) => void;
  /** Delivers everything held so far, then every later push. Returns a function to detach. */
  readonly attach: (listener: TOutputListener) => () => void;
};

export function createSessionOutputBuffer(): TSessionOutputBuffer {
  const pending: Uint8Array[] = [];
  let listener: TOutputListener | undefined;

  return {
    push: (bytes) => {
      if (listener) {
        listener(bytes);
      } else {
        pending.push(bytes);
      }
    },
    attach: (newListener) => {
      listener = newListener;
      for (const bytes of pending.splice(0)) {
        newListener(bytes);
      }
      return () => {
        if (listener === newListener) {
          listener = undefined;
        }
      };
    },
  };
}

export function decodeBase64(data: string): Uint8Array {
  return Uint8Array.from(atob(data), (character) => character.charCodeAt(0));
}
