import { onMounted, onUnmounted } from "vue";

/**
 * Blocks the browser context menu (right-click menu).
 * Useful for desktop apps where the browser context menu is not desired.
 *
 * @param options.inDevMode - Also block in development mode (default: false)
 */
export function useBlockContextMenu(options?: { inDevMode?: boolean }) {
  const shouldBlock = options?.inDevMode || import.meta.env?.PROD === true;

  function blockContextMenu(e: MouseEvent) {
    e.preventDefault();
  }

  onMounted(() => {
    if (shouldBlock) {
      document.addEventListener("contextmenu", blockContextMenu);
    }
  });

  onUnmounted(() => {
    if (shouldBlock) {
      document.removeEventListener("contextmenu", blockContextMenu);
    }
  });
}
