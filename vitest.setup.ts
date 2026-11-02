import '@testing-library/jest-dom/vitest';

// jsdom has no modal dialogs yet; faber-ui's Dialog and AlertDialog need these two.
HTMLDialogElement.prototype.showModal = function showModal(this: HTMLDialogElement) {
  this.setAttribute('open', '');
};
HTMLDialogElement.prototype.close = function close(this: HTMLDialogElement) {
  this.removeAttribute('open');
};
