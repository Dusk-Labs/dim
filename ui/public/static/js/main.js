customElements.define("horizontally-scrollable", class extends HTMLElement {
  constructor () {
    super();

    this.ul = this.querySelector("ul");
    this.size = this.ul.offsetWidth;
    this.scrollPreviousHandler = this.scrollPrevious();
    this.scrollNextHandler = this.scrollNext();
    this.resizeHandler = this.resize();
    this.scrollHandler = this.scroll();
  }

  scroll () {
    return (event) => {
      const scrollLeftPercentage = (this.ul.scrollLeft / this.size) * 100;
      const scrollRightPercentage = ((this.ul.scrollLeft + this.ul.offsetWidth) / this.size) * 100;

      if ( scrollLeftPercentage <= 2 ) {
        this.prev.disabled = true;
      } else {
        this.prev.disabled = false;
      }

      if ( scrollRightPercentage >= 98 ) {
        this.next.disabled = true;
      } else {
        this.next.disabled = false;
      }
    }
  }

  scrollPrevious () {
    return (event) => {
      event.preventDefault();
      const elementWidth = this.ul.querySelector("li").offsetWidth;
      const elementsToScroll = Math.floor(this.ul.offsetWidth / elementWidth) - 1;
      this.ul.scrollLeft = this.ul.scrollLeft - (elementsToScroll * elementWidth);
      this.scrollHandler();
    }
  }

  scrollNext () {
    return (event) => {
      event.preventDefault();
      const elementWidth = this.ul.querySelector("li").offsetWidth;
      const elementsToScroll = Math.floor(this.ul.offsetWidth / elementWidth) - 1;
      this.ul.scrollLeft = this.ul.scrollLeft + (elementsToScroll * elementWidth);
      this.scrollHandler();
    }
  }

  resize () {
    return (event) => {
      this.ul.style.cssText = "width: " + this.parentElement.offsetWidth + "px;";
      this.scrollHandler();
    }
  }

  connectedCallback () {
    // Hack to remove scrollbar visibility and get styles to recompute
    this.ul.style.cssText = "width: " + this.parentElement.offsetWidth + "px;";
    this.ul.scrollLeft = 0;
    this.ul.style.scrollBehavior = "smooth";
    this.style.overflow = "visible";

    window.addEventListener("resize", this.resizeHandler);

    const navButtons = document.createElement("div");
    navButtons.classList.add("navigation");

    this.prev = document.createElement("button");
    this.prev.innerText = "Previous";
    this.prev.disabled = true;
    this.prev.addEventListener("click", this.scrollPreviousHandler);
    this.next = document.createElement("button");
    this.next.addEventListener("click", this.scrollNextHandler);
    this.next.innerText = "Next";
    this.next.disabled = this.ul.scrollLeft >= (this.size - this.ul.offsetWidth);

    navButtons.appendChild(this.prev);
    navButtons.appendChild(this.next);
    this.insertBefore(navButtons, this.querySelector("ul"));

    this.ul.addEventListener("scroll", this.scrollHandler);
  }

  disconnectedCallback () {
    window.removeEventListener("resize", this.resizeHandler);
    this.ul.removeEventListener("scroll", this.scrollHandler);
    this.prev.removeEventListener("click", this.scrollPreviousHandler);
    this.next.removeEventListener("click", this.scrollNextHandler);
    this.prev.parentElement.remove();
  }
});



customElements.define("dim-video-player", class extends HTMLElement {
  constructor () {
    super();

    this.video = this.querySelector("video");
    const dashManifestSource = this.video.querySelector("source[type='application/dash+xml']");
    if (dashManifestSource) {
      this.manifestUri = dashManifestSource.src;
    } else {
      this.manifestUri = video.currentSrc;
    }
  }

  init() {
    return () => {
      this.ui = this.video["ui"];
      this.controls = this.ui.getControls();
      this.player = this.controls.getPlayer();

      // NOTE: Thumbnails will not load if `streaming.useNativeHlsOnSafari` is true.
      // However, seek performance is unreliable when it is false which is why DASH stream sources are preferred. 
      // 
      // player.configure("streaming.useNativeHlsOnSafari", false);

      this.config = {
        controlPanelElements: ["elapsed_and_duration", "play_pause", "spacer", "mute", "volume", "captions", "overflow_menu", "fullscreen"],
        overflowMenuButtons : ["cast", "airplay"]
      }
      this.ui.configure(this.config);

      this.player.addEventListener("error", this.onPlayerErrorEvent);
      this.controls.addEventListener("error", this.onUIErrorEvent);

      this.player.load(this.manifestUri)
        .then(
          () => {
            // Player loaded
          },
          () => {
            this.onPlayerError(e);
          }
        );
    }
  }

  onPlayerError(error) {
    console.error("Error code", error.code, "object", error);
  }

  onPlayerErrorEvent(error) {
    onPlayerError(error);
  }

  onUIErrorEvent(error) {
    onPlayerError(error);
  }

  initFailed () {
    return (errorEvent) => {
      console.error("Unable to load the UI library", errorEvent);
    }
  }

  togglePlayPause() {
    if (this.video.paused) {
      this.video.play();
    } else {
      this.video.pause();
    }
  }

  keyHandler () {
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
    }
  }

  connectedCallback () {
    document.addEventListener("shaka-ui-loaded", this.init());
    document.addEventListener("shaka-ui-load-failed", this.initFailed());

    if (document.querySelector("script[src='/static/js/shaka-player.ui.js']")) return;

    let shaka = document.createElement("script");
    shaka.type = "text/javascript";
    shaka.src = "/static/js/shaka-player.ui.js";
    shaka.addEventListener("load", () => {
      this.video.removeAttribute("controls");
      this.video.setAttribute("autoplay", true);
      initCustomShakaElements();
    });
    document.head.appendChild(shaka);

    document.addEventListener("keydown", this.keyHandler());
  }

  disconnectedCallback () {
    document.removeEventListener("shaka-ui-loaded", this.init());
    document.removeEventListener("shaka-ui-load-failed", this.initFailed());
  }
});

function initCustomShakaElements () {
  /**
   * @extends {shaka.ui.Element}
   * @final
   * @export
   */
  shaka.ui.PresentationCurrentTime = class extends shaka.ui.Element {
    /**
     * @param {!HTMLElement} parent
     * @param {!shaka.ui.Controls} controls
     */
    constructor(parent, controls) {
      super(parent, controls);

      /** @type {!HTMLButtonElement} */
      this.currentTime_ = document.createElement("span");
      this.currentTime_.classList.add('shaka-current-time');
      this.setValue_(this.currentTime_, '0:00');
      this.durationTime_ = document.createElement("span");
      this.durationTime_.classList.add('shaka-duration-time');
      this.setValue_(this.durationTime_, '0:00');
      this.container_ = document.createElement("div");
      this.container_.classList.add('shaka-elapsed-and-duration-time');
      this.container_.appendChild(this.currentTime_);
      this.container_.appendChild(this.durationTime_);
      this.parent.insertAdjacentElement("beforebegin", this.container_);

      this.eventManager.listen(this.controls, 'timeandseekrangeupdated', () => {
        this.updateTime_();
      });
    }

    /** @private */
    setValue_(element, value) {
      // To avoid constant updates to the DOM, which makes debugging more
      // difficult, only set the value if it has changed.  If we don't do this
      // check, the DOM updates constantly, this element flashes in the debugger
      // in Chrome, and you can't make changes in the CSS panel.
      if (value != element.textContent) {
        element.textContent = value;
      }
    }

    buildTimeString(displayTime, showHour) {
      const h = Math.floor(displayTime / 3600);
      const m = Math.floor((displayTime / 60) % 60);
      let s = Math.floor(displayTime % 60);
      if (s < 10) {
        s = '0' + s;
      }
      let text = m + ':' + s;
      if (showHour) {
        if (m < 10) {
          text = '0' + text;
        }
        text = h + ':' + text;
      }
      return text;
    }

    /** @private */
    updateTime_() {
      let displayTime = this.controls.getDisplayTime();
      const seekRange = this.player.seekRange();
      const seekRangeSize = seekRange.end - seekRange.start;

      const showHour = seekRangeSize >= 3600;

      const currentTime = Math.max(0, displayTime - seekRange.start);
      let value = this.buildTimeString(currentTime, showHour);
      if (seekRangeSize) {
        this.setValue_(this.durationTime_, this.buildTimeString(seekRangeSize, showHour));
      }
      this.setValue_(this.currentTime_, value);
    }
  };


  /**
   * @implements {shaka.extern.IUIElement.Factory}
   * @final
   */
  shaka.ui.PresentationCurrentTime.Factory = class {
    /** @override */
    create(rootElement, controls) {
      return new shaka.ui.PresentationCurrentTime(rootElement, controls);
    }
  };

  shaka.ui.Controls.registerElement(
      'elapsed_and_duration', new shaka.ui.PresentationCurrentTime.Factory());
}
