const FALLBACK_CODECS = [
  {
    type: "media-source",
    audio: {
      contentType: "video/mp4;codecs=avc1.640028",
      width: 1920,
      height: 1080,
      bitrate: 128000,
      framerate: 24,
    },
  },
  {
    type: "media-source",
    audio: {
      contentType: "audio/mp4;codecs=mp4a.40.42",
      channels: 2,
      bitrate: 1000,
      sameplerate: 24000,
    },
  },
];

class DimVideoPlayer extends HTMLElement {
  /* eslint-disable no-useless-constructor */
  constructor() {
    super();
  }
  /* eslint-enable no-useless-constructor */

  async connectedCallback() {
    const mediaConfigsScript = this.querySelector(
      "script#media_configurations"
    );
    if (!mediaConfigsScript) {
      throw new Error(
        'No <script id="media_configurations" type="application/json"> found within <dim-video-player>.'
      );
    }
    mediaConfigsScript.remove();
    const mediaConfigs = JSON.parse(mediaConfigsScript.textContent);

    let supportedCodecs = await Promise.all(
      mediaConfigs.map(this.getMediaSupport)
    );
    supportedCodecs = supportedCodecs.filter(
      ({ supported: { supported } }) => supported
    );

    // Some media requires transcoding
    if (mediaConfigs.length !== supportedCodecs.length) {
      // Does this client support transcoding with fallback codecs?
      let supportedFallbacks = await Promise.all(
        mediaConfigs.map(this.getMediaSupport)
      );
      if (FALLBACK_CODECS.length !== supportedFallbacks.length) {
        throw new Error("This client cannot play this media.");
      }
      console.info("This media requires transcoding on this client.");
    }
    const mux_stream_indexes = supportedCodecs
      .map(({ stream_index }) => stream_index)
      .join(",");

    const id = this.dataset.id;

    const initializeManifestURL = `/api/v1/stream/${id}/manifest?mux_stream_indexes=${mux_stream_indexes}`;
    const response = await fetch(initializeManifestURL);
    const { gid } = await response.json();

    const manifestDashUrl = `/api/v1/stream/${gid}/manifest.mpd`;
    let sourceDash = document.createElement("source");
    sourceDash.type = "application/dash+xml";
    sourceDash.src = manifestDashUrl;
    this.manifestUrl = manifestDashUrl;
    // const manifestHlsUrl = `/api/v1/stream/${gid}/manifest.m3u8`;
    // let sourceHls = document.createElement("source");
    // sourceHls.type = "application/vnd.apple.mpegurl";
    // sourceHls.src = manifestHlsUrl;
    this.video = document.createElement("video");
    this.video.dataset.shakaPlayer = "";
    this.video.setAttribute("playsinline", "");
    this.video.setAttribute("controls", "");
    this.video.appendChild(sourceDash);
    // this.video.appendChild(sourceHls);
    let shakaPlayerContainer = document.createElement("div");
    shakaPlayerContainer.dataset.shakaPlayerContainer = "";
    shakaPlayerContainer.appendChild(this.video);
    this.appendChild(shakaPlayerContainer);

    this.loadShakaPlayer();
  }

  initShakaPlayer() {
    return () => {
      // initCustomShakaElements();
      document.addEventListener("keydown", this.keyHandler());

      this.ui = this.video["ui"];
      this.controls = this.ui.getControls();
      this.player = this.controls.getPlayer();
      // this.player.configure("abr.enabled", false);

      // NOTE: Thumbnails will not load if `streaming.useNativeHlsOnSafari` is true.
      // However, seek performance is unreliable when it is false which is why DASH stream sources are preferred.
      //
      // player.configure("streaming.useNativeHlsOnSafari", false);

      this.config = {
        controlPanelElements: [
          "time_and_duration",
          "play_pause",
          "spacer",
          "mute",
          "volume",
          "overflow_menu",
          "fullscreen",
        ],
        overflowMenuButtons: [
          "language",
          "captions",
          "cast",
          "airplay",
          "quality",
        ],
        clearBufferOnQualityChange: false,
      };
      this.ui.configure(this.config);

      this.player.addEventListener("error", this.onPlayerErrorEvent);
      this.controls.addEventListener("error", this.onUIErrorEvent);
    };
  }

  onPlayerError(error) {
    console.error("Error code", error.code, "object", error);
  }

  onPlayerErrorEvent(error) {
    this.onPlayerError(error);
  }

  onUIErrorEvent(error) {
    this.onPlayerError(error);
  }

  initFailed() {
    return (errorEvent) => {
      console.error("Unable to load the UI library", errorEvent);
    };
  }

  togglePlayPause() {
    if (this.video.paused) {
      this.video.play();
    } else {
      this.video.pause();
    }
  }

  keyHandler() {
    return (e) => {
      switch (e.key) {
        case "f":
          if (!!document.fullscreenElement) {
            document.exitFullscreen();
          } else {
            this.video.parentElement.requestFullscreen();
          }
          e.preventDefault();
          break;
        case "k":
          this.togglePlayPause();
          e.preventDefault();
          break;
        case " ":
          this.togglePlayPause();
          e.preventDefault();
          break;
        default:
          break;
      }
    };
  }

  initShakaPlayerFailed() {
    return (errorEvent) => {
      console.error("Unable to load the UI library", errorEvent);
    };
  }

  loadShakaPlayer() {
    this.video.removeAttribute("controls");
    this.video.setAttribute("autoplay", true);

    document.addEventListener("shaka-ui-loaded", this.initShakaPlayer());
    document.addEventListener(
      "shaka-ui-load-failed",
      this.initShakaPlayerFailed()
    );

    if (document.querySelector("script[src='/static/js/shaka-player.ui.js']"))
      return;

    let shakaControlsCss = document.createElement("link");
    shakaControlsCss.rel = "stylesheet";
    shakaControlsCss.type = "text/css";
    shakaControlsCss.href = "/static/js/controls.css";
    document.head.appendChild(shakaControlsCss);

    let shaka = document.createElement("script");
    shaka.type = "text/javascript";
    shaka.src = "/static/js/shaka-player.ui.js";
    document.head.appendChild(shaka);
  }

  async getMediaSupport(mediaConfig) {
    if (!("mediaCapabilities" in navigator)) {
      const supportedMap = {
        supported: false,
        smooth: false,
        powerEfficient: false,
      };
      return Object.assign(mediaConfig, { supported: supportedMap });
    }
    const supportedMap = await navigator.mediaCapabilities.decodingInfo(
      mediaConfig
    );

    return Object.assign(mediaConfig, { supported: supportedMap });
  }

  disconnectedCallback() {
    document.removeEventListener("shaka-ui-loaded", this.initShakaPlayer());
    document.removeEventListener(
      "shaka-ui-load-failed",
      this.initShakaPlayerFailed()
    );
    document.removeEventListener("keydown", this.keyHandler());
  }
}

customElements.define("dim-video-player", DimVideoPlayer);
