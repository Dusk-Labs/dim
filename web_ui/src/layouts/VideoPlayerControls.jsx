import React, { Component } from "react";
import * as Vibrant from "node-vibrant";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import "./VideoPlayerControls.scss";

class VideoPlayerControls extends Component {
    constructor(props) {
        super(props);

        this.progressBar = React.createRef();
        this.overlay = React.createRef();

        this.body = document.getElementsByTagName("body")[0];

        this.onCoverLoad = this.onCoverLoad.bind(this);
        this.toggleVideoPlay = this.toggleVideoPlay.bind(this);
        this.toggleFullscreen = this.toggleFullscreen.bind(this);
        this.toggleVideoVolume = this.toggleVideoVolume.bind(this);
        this.videoSkip = this.videoSkip.bind(this);

        /*
            TODO: centerBox - container that appears in the center to show current actions
            E.G. if user presses pause, it will display a temporary box in the middle with pause glyph.
        */
        this.state = {
            play: true,
            skip: 15,
            current: "00:00:00",
            duration: "00:00:00",
            progressWidth: "0%",
            fullscreen: false,
            mouseMoveTimeout: null,
            mute: false
        };
    }

    async componentDidMount() {
        // ! USE REACT REF
        document.getElementsByTagName("main")[0].style["margin-left"] = "0";
        document.addEventListener("fullscreenchange", this.handlePageFullscreen.bind(this));

        this.props.video.current.addEventListener("timeupdate", this.handleVideoTimeUpdate.bind(this));

        // * VIDEO CONTROLS
        this.props.video.current.addEventListener("play", this.videoPlay.bind(this));
        this.props.video.current.addEventListener("pause", this.videoPause.bind(this));
        this.props.video.current.addEventListener("click", this.toggleVideoPlay.bind(this));
        this.props.video.current.addEventListener("volumechange", this.handleVideoVolumeChange.bind(this));
        this.progressBar.current.addEventListener("click", this.handleProgressbarMouseClick.bind(this));
    }

    async componentWillUnmount() {
        // ! USE REACT REF
        document.removeEventListener("fullscreenchange", this.handlePageFullscreen);

        this.props.video.current.removeEventListener("timeupdate", this.handleVideoTimeUpdate);

        // * VIDEO CONTROLS
        this.props.video.current.removeEventListener("play", this.videoPlay);
        this.props.video.current.removeEventListener("pause", this.videoPause);
        this.props.video.current.removeEventListener("click", this.toggleVideoPlay);
        this.props.video.current.removeEventListener("volumechange", this.handleVideoVolumeChange);
        this.progressBar.current.removeEventListener("click", this.handleProgressbarMouseClick);

        const reqDelTranscode = await fetch(`http://86.21.150.167:8000/api/v1/stream/${this.state.uuid}`, { method: "DELETE"});
        const transcodeDeleted = await reqDelTranscode.json();

        console.log(`[DELETE] TRANSCODING STREAM ${transcodeDeleted}`);
    }

    handleVideoTimeUpdate() {
        console.log("[EVENT] VIDEO TIME UPDATE");

        // FETCH_CARD_OK
        if (this.props.card.fetched && !this.props.card.error) {
            const { duration } = this.props.card.info;
            const { currentTime } = this.props.video.current;
            const width = 100 * (currentTime / duration);

            const { hh, mm, ss } = {
                hh: ("0" + Math.floor(currentTime / 3600)).slice(-2),
                mm: ("0" + Math.floor(currentTime % 3600 / 60)).slice(-2),
                ss: ("0" + Math.floor(currentTime % 3600 % 60)).slice(-2)
            };

            this.setState({
                current: `${hh}:${mm}:${ss}`,
                progressWidth: `${width}%`
            });
        }
    }

    videoPlay() {
        console.log("[EVENT] VIDEO PLAY TRIGGERED");

        this.setState({ play: false });
    }

    videoPause() {
        console.log("[EVENT] VIDEO PAUSE TRIGGERED");

        this.setState({ play: true });
    }

    toggleVideoPlay() {
        console.log("[EVENT] VIDEO PLAY/PAUSE TOGGLED");

        if (this.props.video.current.readyState === 4) {
            this.props.video.current[this.props.video.current.paused ? "play" : "pause"]();
        }
    }

    // FOR WHEN IMPLEMENTING VOLUME SLIDER
    handleVideoVolumeChange(e) { }

    handleProgressbarMouseClick(e) {
        console.log("[EVENT] SEEK BAR CLICKED");

        // FETCH_CARD_OK
        if (this.props.card.fetched && !this.props.card.error) {
            const clicked_pos_x = e.pageX - e.target.offsetLeft;
            const percentage = 100 * clicked_pos_x / e.target.offsetWidth;
            const { duration } = this.props.card.info;

            this.props.video.current.currentTime = percentage * (duration / 100);
        }
    }

    toggleVideoVolume() {
        console.log("[EVENT] VIDEO VOLUME TOGGLED");

        this.setState({
            mute: !this.state.mute
        });

        this.props.video.current.volume = !this.state.mute ? 0 : 1;
    }

    videoSkip(direction) {
        console.log("[EVENT] VIDEO SKIP TRIGGERED");

        if (this.props.video.current.readyState === 4) {
            direction
                ? this.props.video.current.currentTime += this.state.skip
                : this.props.video.current.currentTime -= this.state.skip;
        }
    }

    handlePageFullscreen() {
        console.log("[EVENT] VIDEO FULLSCREEN TRIGGERED");

        this.setState({
            fullscreen: (
                document.webkitIsFullScreen || document.mozFullScreen
            )
        });
    }

    toggleFullscreen() {
        console.log("[EVENT] VIDEO FULLSCREEN TOGGLED");

        if (this.state.fullscreen) {
            const [w3, moz, webkit] = [
                document.exitFullscreen,
                document.mozCancelFullScreen,
                document.webkitExitFullscreen,
            ];

            if (w3) return document.exitFullscreen();
            if (moz) return document.mozCancelFullScreen();
            if (webkit) return document.webkitExitFullscreen();
        } else {
            const [w3, moz, webkit] = [
                document.documentElement.requestFullscreen,
                document.documentElement.mozRequestFullScreen,
                document.documentElement.webkitRequestFullscreen,
            ];

            if (w3) return document.documentElement.requestFullscreen();
            if (moz) return document.documentElement.mozRequestFullScreen();
            if (webkit) return document.documentElement.webkitRequestFullscreen();
        }
    }

    async onCoverLoad(blob) {
        console.log("[EVENT] COVER LOADED");

        const posterBlob = URL.createObjectURL(blob);
        const color = await Vibrant.from(posterBlob).getPalette();

        const root = document.documentElement;
        root.style.setProperty("--accent-background", color.Vibrant.getHex());
        root.style.setProperty("--accent-text", color.Vibrant.getTitleTextColor());
    }

    render() {
        let mediaName;

        // FETCH_CARD_START
        if (this.props.card.fetching) {
            mediaName = "LOADING";
        }

        // FETCH_CARD_ERR
        if (this.props.card.fetched && this.props.card.error) {
            mediaName = "FAILED TO LOAD";
        }

        // FETCH_CARD_OK
        if (this.props.card.fetched && !this.props.card.error) {
            const { name, poster_path } = this.props.card.info;

            mediaName = name;
        }

        return (
            <section className="controls">
                <div className="upper">
                    <div className="left">
                        {this.state.season && this.state.episode &&
                            <div className="se-ep">
                                <p>S{this.state.season}</p>
                                <FontAwesomeIcon icon="circle"/>
                                <p>E{this.state.episode}</p>
                            </div>
                        }
                        <div className="name">
                            <p>{mediaName}</p>
                        </div>
                    </div>
                    <div className="right">
                        <p>{this.state.current}</p>
                        <FontAwesomeIcon icon="circle"/>
                        <p>{this.state.duration}</p>
                    </div>
                </div>
                <div className="center">
                    <div className="video-progress-wrapper" ref={this.progressBar}>
                        <div className="video-progress-inner" style={{width: this.state.progressWidth}}>
                            <div className="video-progress-dragger"/>
                        </div>
                    </div>
                </div>
                <div className="lower">
                    <div className="left">
                        <div className="volume" onClick={this.toggleVideoVolume}>
                            <FontAwesomeIcon icon={this.state.mute ? "volume-mute" : "volume-up"}/>
                        </div>
                        <div className="video-progress-wrapper">
                            <div className="video-progress-inner" style={{width: this.state.progressWidth}}>
                                <div className="video-progress-dragger"/>
                            </div>
                        </div>
                    </div>
                    <div className="middle">
                        <div className="backward">
                            <FontAwesomeIcon icon="backward"/>
                        </div>
                        <div className="skip-backwards" onClick={() => this.videoSkip(false)}>
                            <FontAwesomeIcon icon="fast-backward" onClick={() => this.videoSkip(false)}/>
                        </div>
                        <div className="state" onClick={this.toggleVideoPlay}>
                            <FontAwesomeIcon icon={this.state.play ? "play" : "pause"}/>
                        </div>
                        <div className="skip-forwards" onClick={() => this.videoSkip(true)}>
                            <FontAwesomeIcon icon="fast-forward"/>
                        </div>
                        <div className="forward">
                            <FontAwesomeIcon icon="forward"/>
                        </div>
                    </div>
                    <div className="right">
                        { // ! RE-IMPLEMENT POST-MVP
                        /* <div className="captions">
                            <FontAwesomeIcon icon="closed-captioning"/>
                        </div> */}
                        <div className="fullscreen" onClick={this.toggleFullscreen}>
                            <FontAwesomeIcon icon={this.state.fullscreen ? "compress" : "expand"}/>
                        </div>
                    </div>
                </div>
            </section>
        );
    }
}

export default VideoPlayerControls;