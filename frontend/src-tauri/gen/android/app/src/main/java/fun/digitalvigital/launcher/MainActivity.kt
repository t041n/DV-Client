package `fun`.digitalvigital.launcher

import android.os.Bundle
import android.view.ViewGroup
import android.webkit.WebView
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }

  override fun onResume() {
    super.onResume()
    disableWebViewZoom(window.decorView as? ViewGroup)
  }

  override fun onWindowFocusChanged(hasFocus: Boolean) {
    super.onWindowFocusChanged(hasFocus)
    if (hasFocus) {
      disableWebViewZoom(window.decorView as? ViewGroup)
    }
  }

  private fun disableWebViewZoom(viewGroup: ViewGroup?) {
    if (viewGroup == null) return
    for (i in 0 until viewGroup.childCount) {
      val child = viewGroup.getChildAt(i)
      if (child is WebView) {
        child.settings.setSupportZoom(false)
        child.settings.builtInZoomControls = false
        child.settings.displayZoomControls = false
        child.settings.textZoom = 100
      } else if (child is ViewGroup) {
        disableWebViewZoom(child)
      }
    }
  }
}
