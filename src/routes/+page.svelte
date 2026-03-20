<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import {
    Scissors,
    FolderCog,
    Check,
    LoaderPinwheel,
    Upload,
    X,
  } from "@jis3r/icons";

  // Application State
  let hoverStates = $state({
    dropzone: false,
    cancel: false,
    scissors: false,
    folder: false,
    submit: false,
  });
  let selectedFile: string | null = $state(null);
  let outputDir: string | null = $state(null);
  let finalCutSec: 1 | 2 = $state(1);
  let showCutSelect = $state(false);

  // Status: idle, processing, success, error
  let status: "idle" | "processing" | "success" | "error" = $state("idle");
  let statusMessage: string = $state("");

  // Handle Window Dragging
  function handleDragStart() {
    getCurrentWindow().startDragging();
  }

  // Tauri File Drop Setup
  let unlistenDrop: UnlistenFn;

  onMount(async () => {
    unlistenDrop = await listen("tauri://drag-drop", (event: any) => {
      const paths = event.payload.paths as string[];
      if (paths && paths.length > 0) {
        const file = paths[0];
        if (file.match(/\.(mp4|mov|mkv|avi)$/i)) {
          selectedFile = file;
          status = "idle";
          statusMessage = "";
        } else {
          status = "error";
          statusMessage = "Only MP4, MOV, MKV, AVI videos are supported!";
          setTimeout(() => {
            if (status === "error") {
              status = "idle";
              statusMessage = "";
            }
          }, 3000);
        }
      }
    });

    // We also need to prevent default HTML drop behavior so we don't accidentally load the video in the UI
    document.addEventListener("dragover", (e) => e.preventDefault());
    document.addEventListener("drop", (e) => e.preventDefault());
  });

  onDestroy(() => {
    if (unlistenDrop) unlistenDrop();
  });

  async function handleFolderSelect() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });
      if (selected && typeof selected === "string") {
        outputDir = selected;
      }
    } catch (err) {
      console.error(err);
    }
  }

  async function selectVideo() {
    try {
      const selected = await open({
        multiple: false,
        filters: [{ name: "Video", extensions: ["mp4", "mov", "mkv", "avi"] }],
      });
      if (selected && typeof selected === "string") {
        selectedFile = selected;
      }
    } catch (err) {
      console.error(err);
    }
  }

  async function processVideo() {
    if (!selectedFile) return;

    status = "processing";
    statusMessage = "Editing video...";

    try {
      // Default output to Desktop/Vedmin if not selected
      // We will pass an empty string if null, Rust will handle the absolute Home path.

      const out_path = outputDir || "";

      type ProcessResult = {
        success: boolean;
        message: string;
        output_path: string | null;
      };

      const result: ProcessResult = await invoke("process_video_cmd", {
        inputPath: selectedFile,
        finalCutSec: finalCutSec,
        outputDir: out_path,
      });

      if (result.success) {
        status = "success";
        statusMessage = `Saved to ${result.output_path}`;
        selectedFile = null;
        setTimeout(() => {
          status = "idle";
          statusMessage = "";
        }, 3000);
      } else {
        status = "error";
        statusMessage = result.message;
      }
    } catch (err: any) {
      status = "error";
      statusMessage = err.toString();
    }
  }

  function getStatusColor(s: string) {
    if (s === "processing") return "#A7C080"; // green
    if (s === "success") return "#475258"; // gray (reset)
    if (s === "error") return "#E67E80"; // red
    return "#475258"; // gray (idle)
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="layout" onmousedown={handleDragStart}>
  <!-- MAIN DROP ZONE -->
  <main class="main-area">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div
      class="dropzone glass"
      class:has-file={!!selectedFile}
      onmousedown={(e) => e.stopPropagation()}
      onclick={selectVideo}
    >
      <button
        class="cancel-btn glass"
        class:show={!!selectedFile}
        onclick={(e) => {
          e.stopPropagation();
          selectedFile = null;
          status = "idle";
        }}
        title="Cancelar"
      >
        <X size={16} />
      </button>

      {#if selectedFile}
        <div class="file-info">
          <Upload
            size={48}
            color="#A7C080"
            animate={hoverStates.dropzone || !!selectedFile}
          />
        </div>
      {:else}
        <Upload
          size={48}
          color="var(--text-muted)"
          animate={hoverStates.dropzone}
        />
      {/if}
    </div>
  </main>

  <!-- SIDEBAR -->
  <aside class="sidebar">
    <!-- <div class="logo">V</div> -->

    <div class="actions">
      <!-- Scissors Button -->
      <div class="action-group">
        <button
          class="icon-btn glass"
          disabled={!selectedFile}
          title="Cortar {finalCutSec}s"
          onmouseenter={() => (hoverStates.scissors = true)}
          onmouseleave={() => (hoverStates.scissors = false)}
          onclick={() => selectedFile && (showCutSelect = !showCutSelect)}
          onmousedown={(e) => e.stopPropagation()}
        >
          <Scissors
            size={20}
            animate={hoverStates.scissors && !!selectedFile}
          />
          <span class="badge" class:visible={!!selectedFile}
            >{finalCutSec}s</span
          >
        </button>
        {#if showCutSelect}
          <div
            class="select-menu glass"
            onmousedown={(e) => e.stopPropagation()}
          >
            <button
              class="menu-item"
              class:active={finalCutSec === 1}
              onclick={() => {
                finalCutSec = 1;
                showCutSelect = false;
              }}>1s</button
            >
            <button
              class="menu-item"
              class:active={finalCutSec === 2}
              onclick={() => {
                finalCutSec = 2;
                showCutSelect = false;
              }}>2s</button
            >
          </div>
        {/if}
      </div>

      <!-- Folder Button -->
      <button
        class="icon-btn glass"
        title="Output Folder: {outputDir || 'Default'}"
        onmouseenter={() => (hoverStates.folder = true)}
        onmouseleave={() => (hoverStates.folder = false)}
        onclick={handleFolderSelect}
        onmousedown={(e) => e.stopPropagation()}
        class:active={!!outputDir}
      >
        <FolderCog size={20} animate={hoverStates.folder} />
      </button>

      <!-- Submit Button -->
      <button
        class="icon-btn submit-btn"
        title="Process Video"
        disabled={!selectedFile || status !== "idle"}
        onmouseenter={() => (hoverStates.submit = true)}
        onmouseleave={() => (hoverStates.submit = false)}
        onclick={processVideo}
        onmousedown={(e) => e.stopPropagation()}
      >
        {#if status === "processing"}
          <LoaderPinwheel size={24} animate={true} />
        {:else}
          <Check
            size={24}
            animate={hoverStates.submit && !!selectedFile && status === "idle"}
          />
        {/if}
      </button>
    </div>
  </aside>

  <!-- FOOTER -->
  <footer class="footer">
    <span class="version">VEDMIN V0.0.1</span>
    <div class="status-container">
      <span class="status-msg">{statusMessage}</span>
      <div
        class="led"
        style="background-color: {getStatusColor(status)};"
      ></div>
    </div>
  </footer>
</div>

<style>
  .layout {
    display: grid;
    grid-template-columns: 1fr 80px;
    grid-template-rows: 1fr 40px;
    height: 100%;
    position: relative;
  }

  .main-area {
    grid-column: 1 / 2;
    grid-row: 1 / 2;
    padding: 24px 12px 24px 24px;
    display: flex;
  }

  .dropzone {
    flex: 1;
    border-radius: 12px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    border: 2px dashed var(--surface-border);
    transition: all 0.3s;
    color: var(--text-muted);
    position: relative;
    cursor: pointer;
  }

  .dropzone:hover {
    border-color: rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.08);
  }

  .dropzone.has-file {
    border-color: #a7c080;
    border-style: solid;
    color: #a7c080;
    background: rgba(167, 192, 128, 0.05);
  }

  .cancel-btn {
    position: absolute;
    top: 12px;
    left: 12px;
    width: 32px;
    height: 32px;
    opacity: 0;
    pointer-events: none;
    z-index: 10;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    color: var(--text-muted);
    border: none;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cancel-btn:hover {
    color: #ef4444;
    background: rgba(239, 68, 68, 0.1);
  }

  .cancel-btn.show {
    opacity: 1;
    pointer-events: auto;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .sidebar {
    grid-column: 2 / 3;
    grid-row: 1 / 2;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 24px 0;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin-top: auto;
  }

  .action-group {
    position: relative;
    display: flex;
    justify-content: center;
  }

  .select-menu {
    position: absolute;
    right: 60px;
    top: 0;
    border-radius: 8px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    animation: fadeIn 0.15s ease-out;
    z-index: 10;
  }

  .menu-item {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    padding: 8px 16px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
  }

  .menu-item:hover,
  .menu-item.active {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text-primary);
  }

  .icon-btn {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
    position: relative;
  }

  .icon-btn:hover {
    color: var(--text-primary);
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .icon-btn:active:not(:disabled) :global(svg) {
    transform: scale(0.95);
    transition: transform 0.1s;
  }

  .icon-btn.active {
    color: var(--accent-secondary);
    border-color: var(--accent-secondary);
  }

  .badge {
    position: absolute;
    bottom: 2px;
    right: 2px;
    font-size: 10px;
    font-weight: bold;
    background: var(--surface-border);
    border-radius: 4px;
    padding: 2px 4px;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .badge.visible {
    opacity: 1;
  }

  .submit-btn {
    background: var(--surface-border);
    border: none;
    color: #fff;
  }

  .submit-btn:hover:not(:disabled) {
    transform: translateY(-2px);
  }

  .submit-btn:disabled {
    opacity: 0.5;
    cursor: default;
    background: var(--surface-color);
    color: var(--text-muted);
  }

  .footer {
    grid-column: 1 / 3;
    grid-row: 2 / 3;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 24px;
    font-size: 11px;
    font-weight: 600;
    color: rgba(255, 255, 255, 0.075); /* Opaque white */
    letter-spacing: 1px;
  }

  .status-container {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .status-msg {
    max-width: 400px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    color: rgba(255, 255, 255, 0.075);
    text-transform: uppercase;
  }

  .led {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    transition: all 0.3s;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
      transform: translateX(10px);
    }
    to {
      opacity: 1;
      transform: translateX(0);
    }
  }
</style>
