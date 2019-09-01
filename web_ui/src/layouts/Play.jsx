import React, { Component } from "react";
import HLS from "hls.js";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import endgame1080 from "../assets/endgame_1080p.mp4";

import "./Play.scss";
import LazyImage from "../helpers/LazyImage.jsx";

class Play extends Component {
    constructor(props) {
        super(props);

        this.video = React.createRef();
        this.progressBar = React.createRef();
        this.overlay = React.createRef();

        this.body = document.getElementsByTagName("body")[0];

        this.handleMouseMove = this.handleMouseMove.bind(this);
        this.handleProgressbarMouseClick = this.handleProgressbarMouseClick.bind(this);
        this.toggleVideoPlay = this.toggleVideoPlay.bind(this);
        this.handleVideoLoaded = this.handleVideoLoaded.bind(this);
        this.handleVideoTimeUpdate = this.handleVideoTimeUpdate.bind(this);
        this.videoSkip = this.videoSkip.bind(this);
        this.toggleFullscreen = this.toggleFullscreen.bind(this);
        this.handlePageFullscreen = this.handlePageFullscreen.bind(this);
        this.handleVideoVolumeChange = this.handleVideoVolumeChange.bind(this);
        this.toggleVideoVolume = this.toggleVideoVolume.bind(this);

        this.state = {
            name: "Avengers: Endgame",
            cover: "http://t2.gstatic.com/images?q=tbn:ANd9GcQA_-tL18_rj9zEcjN6n41NEaJm-kRNF9UeOtvksZ4z_OW6jRA9",
            play: true,
            skip: 15,
            current: "00:00:00",
            duration: "00:00:00",
            endsAt: null,
            progressWidth: "0%",
            fullscreen: false,
            mouseMoveTimeout: null,
            season: null,
            episode: null,
            mute: false,
        };
    }

    // TODO: mousemove, .overlay -> background: radial-gradient(circle, transparent 50%, black 10%);
    componentDidMount() {
        const main = document.getElementsByTagName("main")[0];
        main.style["margin-left"] = "0";

        this.video.current.addEventListener("click", this.toggleVideoPlay);
        this.video.current.addEventListener("loadeddata", this.handleVideoLoaded);
        this.video.current.addEventListener("timeupdate", this.handleVideoTimeUpdate);
        this.video.current.addEventListener("volumechange", this.handleVideoVolumeChange);
        document.addEventListener("fullscreenchange", this.handlePageFullscreen);
        this.video.current.addEventListener("mousemove", this.handleMouseMove);
        this.progressBar.current.addEventListener("click", this.handleProgressbarMouseClick);

        // ! WILL SWITCH TO THIS WHEN PLAYER DESIGN IS SOMEWHAT DONE.
        // const id = "66232264-6baf-4dc6-bf3a-6bc6cc6a0131";

        // const config = {
        //     autoStartLoad: true,
        //     startPosition: 0,
        //     debug: false,
        // };

        // const source = `http://86.21.150.167:8000/api/v1/stream/static/${id}/index.m3u8`;

        // // ! FIXME: USING OLD VER (0.8.8) (OUTDATED)
        // if (HLS.isSupported()) {
        //     const hls = new HLS(config);

        //     hls.attachMedia(this.video.current);

        //     hls.on(HLS.Events.MEDIA_ATTACHED, function () {
        //         hls.loadSource(source);

        //         hls.on(HLS.Events.MANIFEST_PARSED, function (event, data) {
        //             this.video.current.currentTime = 0;
        //             this.video.current.play();
        //         });
        //     });

        //     window.hls = hls;
        //     window.player = this.video.current;
        // }
        // !
    }

    componentWillUnmount() {
        const main = document.getElementsByTagName("main")[0];
        main.style["margin-left"] = "300px";

        this.video.current.removeEventListener("click");
        this.video.current.removeEventListener("loadeddata")
        this.video.current.removeEventListener("timeupdate");
        this.video.current.removeEventListener("mousemove");
        this.progressBar.current.removeEventListener("click");
    }

    handleProgressbarMouseClick(e) {
        const clicked_pos_x = e.pageX - e.target.offsetLeft;
        const percentage = 100 * clicked_pos_x / e.target.offsetWidth;

        this.video.current.currentTime = percentage * (this.video.current.duration / 100);
    }

    // FOR WHEN IMPLEMENTING VOLUME SLIDER
    handleVideoVolumeChange(e) { }

    toggleVideoVolume() {
        this.setState({
            mute: !this.state.mute
        });

        this.video.current.volume = !this.state.mute ? 0 : 1;
    }

    handleMouseMove() {
        if (this.state.mouseMoveTimeout !== null) {
            clearTimeout(this.state.mouseMoveTimeout);

            this.setState({
                mouseMoveTimeout: null
            });

            this.overlay.current.style.opacity = 1;
            this.body.style.cursor = "default";
        } else {
            this.setState({
                mouseMoveTimeout: setTimeout(() => {
                    this.overlay.current.style.opacity = 0;
                    this.body.style.cursor = "none";
                }, 1000)
            });
        }
    }

    handleVideoLoaded() {
        const { hh, mm, ss } = {
            hh: ("0" + Math.floor(this.video.current.duration / 3600)).slice(-2),
            mm: ("0" + Math.floor(this.video.current.duration % 3600 / 60)).slice(-2),
            ss: ("0" + Math.floor(this.video.current.duration % 3600 % 60)).slice(-2)
        };

        const currentDate = new Date();
        currentDate.setSeconds(currentDate.getSeconds() + this.video.current.duration);

        this.setState({
            duration: `${hh}:${mm}:${ss}`,
            endsAt: currentDate.toLocaleString("en-US", { hour: "numeric", minute: "numeric", hour12: true })
        });
    }

    handleVideoTimeUpdate() {
        const width = 100 * (this.video.current.currentTime / this.video.current.duration);

        const { hh, mm, ss } = {
            hh: ("0" + Math.floor(this.video.current.currentTime / 3600)).slice(-2),
            mm: ("0" + Math.floor(this.video.current.currentTime % 3600 / 60)).slice(-2),
            ss: ("0" + Math.floor(this.video.current.currentTime % 3600 % 60)).slice(-2)
        };

        this.setState({
            current: `${hh}:${mm}:${ss}`,
            progressWidth: `${width}%`
        });
    }

    toggleVideoPlay() {
        this.setState({
            play: !this.state.play,
        });

        if (this.video.current.readyState === 4) {
            this.video.current[this.video.current.paused ? "play" : "pause"]();
        }
    }

    videoSkip(direction) {
        if (this.video.current.readyState === 4) {
            direction
                ? this.video.current.currentTime += this.state.skip
                : this.video.current.currentTime -= this.state.skip;
        }
    }

    handlePageFullscreen() {
        this.setState({
            fullscreen: (
                document.webkitIsFullScreen
                || document.mozFullScreen
                || document.msFullscreenElement
            )
        });
    }

    toggleFullscreen() {
        if (this.state.fullscreen) {
            const [w3, moz, webkit, ms] = [
                document.exitFullscreen,
                document.mozCancelFullScreen,
                document.webkitExitFullscreen,
                document.msExitFullscreen
            ];

            if (w3) document.exitFullscreen();
            if (moz) document.mozCancelFullScreen();
            if (webkit) document.webkitExitFullscreen();
            if (ms) document.msExitFullscreen();
        } else {
            const [w3, moz, webkit, ms] = [
                document.documentElement.requestFullscreen,
                document.documentElement.mozRequestFullScreen,
                document.documentElement.webkitRequestFullscreen,
                document.documentElement.msRequestFullscreen
            ];

            if (w3) document.documentElement.requestFullscreen();
            if (moz) document.documentElement.mozRequestFullScreen();
            if (webkit) document.documentElement.webkitRequestFullscreen();
            if (ms) document.documentElement.msRequestFullscreen();
        }
    }

    render() {
        return (
            <main>
                <div className="video-wrapper">
                    <video ref={this.video} src={endgame1080}></video>
                    <div className="overlay" ref={this.overlay}>
                        <section className="cover">
                            <LazyImage alt="cover" src={this.state.cover}/>
                            {this.state.season && this.state.episode &&
                                <p>{this.state.name}</p>
                            }
                        </section>
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
                                        <p>{this.state.name}</p>
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
                                        <div className="video-progress-dragger"></div>
                                    </div>
                                </div>
                            </div>
                            <div className="lower">
                                <div className="left">
                                    <div className="volume" onClick={this.toggleVideoVolume}>
                                        <FontAwesomeIcon icon={this.state.mute ? "volume-mute" : "volume-up"}/>
                                    </div>
                                </div>
                                <div className="middle">
                                    <div className="backward">
                                        <FontAwesomeIcon icon="backward"/>
                                    </div>
                                    <div className="skip-backwards" onClick={() => this.videoSkip(false)}>
                                        <FontAwesomeIcon icon="fast-backward"/>
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
                                    <div className="fullscreen" onClick={this.toggleFullscreen}>
                                        <FontAwesomeIcon icon={this.state.fullscreen ? "compress" : "expand"}/>
                                    </div>
                                </div>
                            </div>
                        </section>
                        <section className="ends-at">
                            <p>ENDS AT</p>
                            <p>{this.state.endsAt}</p>
                        </section>
                    </div>
                </div>
            </main>
        );
    }
}

export default Play;