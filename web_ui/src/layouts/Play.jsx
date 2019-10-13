import React, { Component } from "react";
import * as Vibrant from "node-vibrant";
import HLS from "hls.js";
import { NavLink } from "react-router-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import "./Play.scss";
import LazyImage from "../helpers/LazyImage.jsx";

class Play extends Component {
    constructor(props) {
        super(props);

        this.video = React.createRef();
        this.progressBar = React.createRef();
        this.overlay = React.createRef();
        this.navLinks = React.createRef();
        this.navPages = React.createRef();

        this.body = document.getElementsByTagName("body")[0];

        this.onCoverLoad = this.onCoverLoad.bind(this);
        this.toggleVideoPlay = this.toggleVideoPlay.bind(this);
        this.handleVideoLoaded = this.handleVideoLoaded.bind(this);
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
            mute: false,
            navIndex: -1
        };
    }

    // TODO: mousemove, .overlay -> background: radial-gradient(circle, transparent 50%, black 10%);
    async componentDidMount() {
        document.getElementsByTagName("main")[0].style["margin-left"] = 0;

        this.video.current.addEventListener("loadeddata", this.handleVideoLoaded.bind(this));
        this.video.current.addEventListener("timeupdate", this.handleVideoTimeUpdate.bind(this));
        this.video.current.addEventListener("mousemove", this.handleMouseMove.bind(this));

        this.video.current.addEventListener("play", this.videoPlay.bind(this));
        this.video.current.addEventListener("pause", this.videoPause.bind(this));
        this.video.current.addEventListener("click", this.toggleVideoPlay.bind(this));
        this.video.current.addEventListener("volumechange", this.handleVideoVolumeChange.bind(this));

        this.progressBar.current.addEventListener("click", this.handleProgressbarMouseClick.bind(this));
        document.addEventListener("fullscreenchange", this.handlePageFullscreen.bind(this));

        // const reqSourceID = await fetch(`http://86.21.150.167:8000/api/v1/stream/start/${this.props.match.params.id}`);
        // const sourceID = await reqSourceID.json();

        const sourceID = "66232264-6baf-4dc6-bf3a-6bc6cc6a0131"; // ! TEMPORARY - FOR TESTING
        const source = `http://86.21.150.167:8000/api/v1/stream/static/${sourceID}/index.m3u8`;

        const config = {
            autoStartLoad: true,
            startPosition: 0,
            debug: false,
        };

        // ! FIXME: USING OLD VER (0.8.8) (OUTDATED)
        if (HLS.isSupported()) {
            const hls = new HLS(config);

            hls.attachMedia(this.video.current);
            hls.on(HLS.Events.MEDIA_ATTACHED, () => hls.loadSource(source));

            window.hls = hls;
            window.player = this.video.current;
        }
        // !

        // ! TO BE REPLACED WITH API
        let versions = [
            {
                file: "FILENAME",
                codec: "H624",
                videoBitrate: "200 KB/S",
                audio: "AAC",
                audioBitrate: "192 KB/S",
                library: "2"
            },
            {
                file: "FILENAME",
                codec: "H624",
                videoBitrate: "200 KB/S",
                audio: "AAC",
                audioBitrate: "192 KB/S",
                library: "3"
            },
            {
                file: "FILENAME",
                codec: "H624",
                videoBitrate: "200 KB/S",
                audio: "AAC",
                audioBitrate: "192 KB/S",
                library: "4"
            },
            {
                file: "FILENAME",
                codec: "H624",
                videoBitrate: "200 KB/S",
                audio: "AAC",
                audioBitrate: "192 KB/S",
                library: "5"
            }
        ];
        // !

        versions = versions.map((
            { file, codec, videoBitrate, audio, audioBitrate, library }, i
        ) => (
            <NavLink className="version" key={i} to="#">
                <FontAwesomeIcon icon="file-video"/>
                <p>{file} - {codec}@{videoBitrate} {audio}@{audioBitrate} - Library {library}</p>
            </NavLink>
        ));

        this.setState({
            name: "Gravity",
            cover: "https://images-na.ssl-images-amazon.com/images/I/41qngCO1gzL.jpg",
            versions
        });
    }

    componentWillUnmount() {
        this.video.current.removeEventListener("loadeddata", this.handleVideoLoaded);
        this.video.current.removeEventListener("timeupdate", this.handleVideoTimeUpdate);
        this.video.current.removeEventListener("mousemove", this.handleMouseMove);

        this.video.current.removeEventListener("play", this.videoPlay);
        this.video.current.removeEventListener("pause", this.videoPause);
        this.video.current.removeEventListener("click", this.toggleVideoPlay);
        this.video.current.removeEventListener("volumechange", this.handleVideoVolumeChange);

        this.progressBar.current.removeEventListener("click", this.handleProgressbarMouseClick);
        document.removeEventListener("fullscreenchange", this.handlePageFullscreen);
    }

    handleVideoLoaded() {
        const currentDate = new Date();
        const { duration } = this.video.current;

        const { hh, mm, ss } = {
            hh: ("0" + Math.floor(duration / 3600)).slice(-2),
            mm: ("0" + Math.floor(duration % 3600 / 60)).slice(-2),
            ss: ("0" + Math.floor(duration % 3600 % 60)).slice(-2)
        };

        currentDate.setSeconds(currentDate.getSeconds() + duration);

        this.setState({
            duration: `${hh}:${mm}:${ss}`,
            endsAt: currentDate.toLocaleString("en-US", { hour: "numeric", minute: "numeric", hour12: true }),
            resolution: "1080p"
        });
    }

    handleVideoTimeUpdate() {
        const { currentTime, duration } = this.video.current;
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

    handleMouseMove() {
        if (this.state.mouseMoveTimeout !== null) {
            clearTimeout(this.state.mouseMoveTimeout);

            this.setState({
                mouseMoveTimeout: null
            });

            this.overlay.current.style.opacity = 1;
            this.body.style.cursor = "default";
        } else {
            if (!this.state.play) {
                this.setState({
                    mouseMoveTimeout: setTimeout(() => {
                        this.overlay.current.style.opacity = 0;
                        this.body.style.cursor = "none";
                    }, 2000)
                });
            }
        }
    }

    videoPlay() {
        this.setState({ play: false });
    }

    videoPause() {
        this.setState({ play: true });
    }

    toggleVideoPlay() {
        if (this.video.current.readyState === 4) {
            this.video.current[this.video.current.paused ? "play" : "pause"]();
        }
    }

    // FOR WHEN IMPLEMENTING VOLUME SLIDER
    handleVideoVolumeChange(e) { }

    handleProgressbarMouseClick(e) {
        const clicked_pos_x = e.pageX - e.target.offsetLeft;
        const percentage = 100 * clicked_pos_x / e.target.offsetWidth;

        this.video.current.currentTime = percentage * (this.video.current.duration / 100);
    }

    toggleVideoVolume() {
        this.setState({
            mute: !this.state.mute
        });

        this.video.current.volume = !this.state.mute ? 0 : 1;
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
                document.webkitIsFullScreen || document.mozFullScreen
            )
        });
    }

    toggleFullscreen() {
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
        const posterBlob = URL.createObjectURL(blob);
        const color = await Vibrant.from(posterBlob).getPalette();

        const root = document.documentElement;
        root.style.setProperty("--accent-background", color.Vibrant.getHex());
        root.style.setProperty("--accent-text", color.Vibrant.getTitleTextColor());
    }

    navSelect(e, index) {
        if (this.state.navIndex === index) {
            e.target.classList.add("inActive");
            e.target.classList.remove("active");

            this.navPages.current.children[index].classList.add("hidden");
            this.navPages.current.children[index].classList.remove("shown");

            return this.setState({navIndex: -1});
        }

        this.setState({navIndex: index});

        // eslint-disable-next-line
        for (let [i, navLink] of [...this.navLinks.current.children].entries()) {
            if (i === index) {
                navLink.classList.add("active");
                navLink.classList.remove("inActive");
                continue;
            };

            navLink.classList.add("inActive");
            navLink.classList.remove("active");
        }

        // eslint-disable-next-line
        for (let [i, navPage] of [...this.navPages.current.children].entries()) {
            if (i === index) {
                navPage.classList.add("shown");
                navPage.classList.remove("hidden");
                continue;
            };

            navPage.classList.add("hidden");
            navPage.classList.remove("shown");
        }
    }

    render() {
        const coverLoading = (
            <div className="placeholder">
                <div className="spinner"></div>
            </div>
        );

        return (
            <main>
                <div className="video-wrapper">
                    <video ref={this.video}></video>
                    <div className="overlay" ref={this.overlay}>
                        <section className="cover">
                            <div className="card-wrapper">
                                <div className="card">
                                <a href={this.state.cover} rel="noopener noreferrer" target="_blank">
                                    <LazyImage alt="cover" src={this.state.cover} onLoad={this.onCoverLoad} loading={coverLoading}/>
                                </a>
                                </div>
                            </div>
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
                                    <div className="video-progress-wrapper">
                                        <div className="video-progress-inner" style={{width: this.state.progressWidth}}>
                                            <div className="video-progress-dragger"></div>
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
                                    <div className="captions">
                                        <FontAwesomeIcon icon="closed-captioning"/>
                                    </div>
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
                        <section ref={this.navLinks} className="video-nav">
                            <p onClick={(e) => this.navSelect(e, 0)} className="inActive">VERSIONS</p>
                            <p onClick={(e) => this.navSelect(e, 1)} className="inActive">CAST</p>
                            <p onClick={(e) => this.navSelect(e, 2)} className="inActive">DIRECTORS</p>
                            <p onClick={(e) => this.navSelect(e, 3)} className="inActive">MEDIA INFO</p>
                        </section>
                        <section ref={this.navPages} className="pages">
                            <div className="page hidden select-version">
                                <h3>VERSIONS</h3>
                                <div className="versions">
                                    {this.state.versions}
                                </div>
                            </div>
                            <div className="page hidden cast">
                                <h3>CAST</h3>
                            </div>
                            <div className="page hidden directors">
                                <h3>DIRECTORS</h3>
                            </div>
                            <div className="page hidden media-info">
                                <h3>MEDIA INFO</h3>
                            </div>
                        </section>
                    </div>
                </div>
            </main>
        );
    }
}

export default Play;