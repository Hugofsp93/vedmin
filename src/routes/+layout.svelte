<script>
  import "../app.css";
</script>

<div class="app-layout" data-tauri-drag-region>
  <div class="content">
    <slot />
  </div>
</div>

<style>
  .app-layout {
    width: 100%;
    height: 100%;
    background-color: var(--bg-color);
    display: flex;
    flex-direction: column;
    position: relative;
    padding-top: var(--traffic-light-space);
    border-radius: 10px; /* macOS rounded corners */
  }

  /* Space for macOS traffic lights */
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    /* padding-top: var(--traffic-light-space); */
    height: 100%;
  }
</style>
