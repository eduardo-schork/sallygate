import { relaunch } from '@tauri-apps/plugin-process';
import { check } from '@tauri-apps/plugin-updater';

/** A newer version, ready to download. */
export type TAvailableUpdate = {
  readonly version: string;
  /** Downloads and checks the signed update, installs it and restarts the app. */
  readonly install: (onProgress: (share: number) => void) => Promise<void>;
};

/** The app's own updates (ADR-0024). The only UI code that talks to Tauri for them. */
export type TUpdatesGateway = {
  /** The newer version, if the latest release is newer than this app. */
  readonly checkForUpdate: () => Promise<TAvailableUpdate | undefined>;
};

export const tauriUpdatesGateway: TUpdatesGateway = {
  checkForUpdate: async () => {
    const update = await check();
    if (!update) {
      return undefined;
    }
    return {
      version: update.version,
      install: async (onProgress) => {
        let total = 0;
        let downloaded = 0;
        await update.downloadAndInstall((event) => {
          if (event.event === 'Started') {
            total = event.data.contentLength ?? 0;
          } else if (event.event === 'Progress') {
            downloaded += event.data.chunkLength;
            onProgress(total > 0 ? downloaded / total : 0);
          }
        });
        await relaunch();
      },
    };
  },
};
