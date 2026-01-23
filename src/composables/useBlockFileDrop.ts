import { onMounted, onUnmounted } from 'vue'

/**
 * Blocks file drag-and-drop on the window.
 * Prevents users from accidentally dropping files into the app,
 * which would navigate away from the application.
 */
export function useBlockFileDrop() {
  function blockFileDrop(e: DragEvent) {
    e.preventDefault()
    if (e.dataTransfer) {
      e.dataTransfer.effectAllowed = 'none'
      e.dataTransfer.dropEffect = 'none'
    }
  }

  onMounted(() => {
    window.addEventListener('dragenter', blockFileDrop)
    window.addEventListener('dragover', blockFileDrop)
    window.addEventListener('drop', blockFileDrop)
  })

  onUnmounted(() => {
    window.removeEventListener('dragenter', blockFileDrop)
    window.removeEventListener('dragover', blockFileDrop)
    window.removeEventListener('drop', blockFileDrop)
  })
}
