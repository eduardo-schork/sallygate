/** What to type into a shell to run a snippet: each line ends with Enter, as if typed. */
export function snippetInput(command: string): string {
  return `${command.replace(/\r?\n/gu, '\r')}\r`;
}
