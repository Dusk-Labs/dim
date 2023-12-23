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
    this.style.overflow = "visible";
    this.ul.style.cssText = "overflow-y: hidden; width: " + this.parentElement.offsetWidth + "px;";

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
