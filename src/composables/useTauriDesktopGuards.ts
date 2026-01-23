import { useBlockFileDrop } from './useBlockFileDrop'
import { useBlockContextMenu } from './useBlockContextMenu'

export interface TauriDesktopGuardsOptions {
  /**
   * Block file drag-and-drop
   * @default true
   */
  blockFileDrop?: boolean

  /**
   * Block context menu in production
   * @default true
   */
  blockContextMenu?: boolean

  /**
   * Block context menu even in development
   * @default false
   */
  blockContextMenuInDev?: boolean
}

/**
 * Convenience composable that applies all desktop guards.
 * Use this in your app.vue or a layout to apply all guards at once.
 *
 * @example
 * ```vue
 * <script setup>
 * useTauriDesktopGuards()
 * </script>
 * ```
 */
export function useTauriDesktopGuards(options?: TauriDesktopGuardsOptions) {
  const opts = {
    blockFileDrop: true,
    blockContextMenu: true,
    blockContextMenuInDev: false,
    ...options,
  }

  if (opts.blockFileDrop) {
    useBlockFileDrop()
  }

  if (opts.blockContextMenu) {
    useBlockContextMenu({ inDevMode: opts.blockContextMenuInDev })
  }
}
