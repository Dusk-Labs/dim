import React, { Component } from "react";
import { connect } from "react-redux";
import * as Vibrant from "node-vibrant";
import HLS from "hls.js";

import { fetchMediaInfo } from "../actions/cardActions.js";
import { startTranscode } from "../actions/videoPlayerActions.js";

import LazyImage from "../helpers/LazyImage.jsx";
import VideoPlayerControls from "./VideoPlayerControls.jsx";

import "./VideoPlayer.scss";

class VideoPlayer extends Component {
    constructor(props) {
        super(props);

        this.video = React.createRef();
        this.overlay = React.createRef();

        // ! RE-IMPLEMENT POST-MVP
        // this.navLinks = React.createRef();
        // this.navPages = React.createRef();

        this.body = document.getElementsByTagName("body")[0];

        this.onCoverLoad = this.onCoverLoad.bind(this);
        this.handleVideoLoaded = this.handleVideoLoaded.bind(this);
        this.updateSeekTo = this.updateSeekTo.bind(this);

        /*
            TODO: centerBox - container that appears in the center to show current actions
            E.G. if user presses pause, it will display a temporary box in the middle with pause glyph.
        */
        this.state = {
            seekTo: 0,
            mouseMoveTimeout: null,
            navIndex: -1
        };
    }

    async componentDidMount() {
        // ! USE REACT REF
        document.querySelector("meta[name='theme-color']").setAttribute("content", "#000000");
        document.getElementsByTagName("main")[0].style["margin-left"] = "0";

        this.video.current.addEventListener("loadeddata", this.handleVideoLoaded.bind(this));
        this.video.current.addEventListener("mousemove", this.handleMouseMove.bind(this));

        const { id } = this.props.match.params;

        this.props.fetchMediaInfo(id);
        this.props.startTranscode(id);
    }

    async componentWillUnmount() {
        // ! USE REACT REF
        document.querySelector("meta[name='theme-color']").setAttribute("content", "#333333");

        this.video.current.removeEventListener("loadeddata", this.handleVideoLoaded);
        this.video.current.removeEventListener("mousemove", this.handleMouseMove);
    }

    componentDidUpdate(prevProps) {
        if (prevProps.stream.start_transcode.fetched !== this.props.stream.start_transcode.fetched) {
            if (!this.state.uuid) {
                // START_TRANSCODE_START
                if (this.props.stream.start_transcode.fetching) {
                    console.log("[FETCHING] START TRANSCODE");
                }

                // START_TRANSCODE_ERR
                if (this.props.stream.start_transcode.fetched && this.props.stream.start_transcode.error) {
                    console.log("[ERR] START TRANSCODE", this.props.stream.start_transcode);
                }

                // START_TRANSCODE_OK
                if (this.props.stream.start_transcode.fetched && !this.props.stream.start_transcode.error) {
                    console.log("[OK] START TRANSCODE");
                    const { uuid } = this.props.stream.start_transcode;

                    const ws = new WebSocket(`ws://86.21.150.167:3012/events/stream/${uuid}`);

                    ws.addEventListener("message", ({data}) => {
                        const payload = JSON.parse(data);

                        if (payload.type === "EventStreamStats") {
                            console.log("[WS] [EventStreamStats] FRAME", payload.frame);

                            if (payload.frame >= 700) {
                                console.log("[WS] [EventStreamStats] ENOUGH FRAMES, CLOSING CONNECTION.");

                                this.fetchFile(uuid);
                                ws.close();
                            }
                        }
                    });

                    this.setState({uuid});
                }
            }
        }

        if (prevProps.stream.del_transcode.fetched !== this.props.stream.del_transcode.fetched) {
            // DEL_TRANSCODE_START
            if (this.props.stream.del_transcode.fetching) {
                console.log("[DELETING] TRANSCODE STREAM.");
            }

            // DEL_TRANSCODE_ERR
            if (this.props.stream.del_transcode.fetched && this.props.stream.del_transcode.error) {
                console.log("[ERR] DELETING TRANSCODE STREAM.", this.props.stream.del_transcode);
            }

            // DEL_TRANSCODE_OK
            if (this.props.stream.del_transcode.fetched && !this.props.stream.del_transcode.error) {
                console.log("[OK] TRANSCODE DELETED.", this.state.seekTo);
                this.setState({uuid: undefined});
                this.hls.destroy();
                this.props.startTranscode(this.props.match.params.id, `?seek=${this.state.seekTo}`);
            }
        }

        // FETCH_CARD_OK
        if (!this.state.endsAt && this.props.card.fetched && !this.props.card.error) {
            const currentDate = new Date();
            const { duration } = this.props.card.info;

            currentDate.setSeconds(currentDate.getSeconds() + duration);

            const endsAt = currentDate.toLocaleString("en-US", { hour: "numeric", minute: "numeric", hour12: true });

            this.setState({endsAt});
        }
    }

    fetchFile(uuid) {
        console.log("[FETCH FILE] BEGINNING TO FETCH TRANSCODED FILE.");

        // ! FIXME: USING OLD VER (0.8.8) (OUTDATED)
        if (HLS.isSupported()) {
            const config = {
                autoStartLoad: true,
                startPosition: this.state.seekTo,
                debug: false,
            };

            const source = `http://86.21.150.167:8000/api/v1/stream/static/${uuid}/index.m3u8`;

            this.hls = new HLS(config);

            this.hls.attachMedia(this.video.current);

            this.hls.on(HLS.Events.MEDIA_ATTACHED, () => {
                console.log("[EVENT] VID COMPONENT AND HLS BOUND.");

                this.hls.loadSource(source);

                this.hls.on(HLS.Events.MANIFEST_PARSED, (_, data) => {
                    console.log("[HLS] MANIFEST LOADED, FOUND " + data.levels.length + " QUALITY LEVEL.");
                });
            });

            this.hls.on(HLS.Events.ERROR, (_, data) => {
                if (data.fatal) {
                    switch(data.type) {
                        case HLS.ErrorTypes.NETWORK_ERROR:
                            console.log("[HLS] FATAL NETWORK ERROR, TRYING TO RECOVER.");
                            this.hls.startLoad();
                            break;
                        case HLS.ErrorTypes.MEDIA_ERROR:
                            console.log("[HLS] FATAL MEDIA ERROR, TRYING TO RECOVER.");
                            this.hls.recoverMediaError();
                            break;
                        default:
                            console.log("[HLS] CANNOT RECOVER, DESTROYING.");
                            this.hls.destroy();
                            break;
                        }
                    }
            });
        }
        // !
    }

    updateSeekTo(secs) {
        this.setState({
            seekTo: secs
        });
    }

    handleVideoLoaded() {
        console.log("[EVENT] VIDEO LOADED");
        this.video.current.play();
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
            if (this.video.current.readyState >= 1 && !this.video.current.paused) {
                const mouseMoveTimeout = setTimeout(_ => {
                    this.overlay.current.style.opacity = 0;
                    this.body.style.cursor = "none";
                }, 3000);

                this.setState({
                    mouseMoveTimeout
                });
            }
        }
    }

    async onCoverLoad(blob) {
        const posterBlob = URL.createObjectURL(blob);
        const color = await Vibrant.from(posterBlob).getPalette();

        const root = document.documentElement;
        root.style.setProperty("--accent-background", color.Vibrant.getHex());
        root.style.setProperty("--accent-text", color.Vibrant.getTitleTextColor());
    }

    // ! RE-IMPLEMENT POST-MVP
    // navSelect(e, index) {
    //     if (this.state.navIndex === index) {
    //         e.target.classList.add("inActive");
    //         e.target.classList.remove("active");

    //         this.navPages.current.children[index].classList.add("hidden");
    //         this.navPages.current.children[index].classList.remove("shown");

    //         return this.setState({navIndex: -1});
    //     }

    //     this.setState({navIndex: index});

    //     // eslint-disable-next-line
    //     for (let [i, navLink] of [...this.navLinks.current.children].entries()) {
    //         if (i === index) {
    //             navLink.classList.add("active");
    //             navLink.classList.remove("inActive");
    //             continue;
    //         };

    //         navLink.classList.add("inActive");
    //         navLink.classList.remove("active");
    //     }

    //     // eslint-disable-next-line
    //     for (let [i, navPage] of [...this.navPages.current.children].entries()) {
    //         if (i === index) {
    //             navPage.classList.add("shown");
    //             navPage.classList.remove("hidden");
    //             continue;
    //         };

    //         navPage.classList.add("hidden");
    //         navPage.classList.remove("shown");
    //     }
    // }

    render() {
        let posterPath;

        // FETCH_CARD_OK
        if (this.props.card.fetched && !this.props.card.error) {
            const { name, poster_path } = this.props.card.info;

            posterPath = poster_path;
            document.title = `Dim - Playing '${name}'`;
        }

        return (
            <div className="video-wrapper">
                <video ref={this.video}/>
                <div className="overlay" ref={this.overlay}>
                    <section className="cover">
                        <div className="card-wrapper">
                            <div className="card">
                                <LazyImage alt="cover" src={posterPath} onLoad={this.onCoverLoad}/>
                            </div>
                        </div>
                    </section>
                    {this.video.current &&
                        <VideoPlayerControls video={this.video.current} card={this.props.card} updateSeekTo={this.updateSeekTo}/>
                    }
                    <section className="ends-at">
                        <p>ENDS AT</p>
                        <p>{this.state.endsAt}</p>
                    </section>
                    { // ! RE-IMPLEMENT POST-MVP
                    /* <section ref={this.navLinks} className="video-nav">
                        <p onClick={(e) => this.navSelect(e, 0)} className="inActive">VERSIONS</p>
                        <p onClick={(e) => this.navSelect(e, 1)} className="inActive">CAST</p>
                        <p onClick={(e) => this.navSelect(e, 2)} className="inActive">DIRECTORS</p>
                        <p onClick={(e) => this.navSelect(e, 3)} className="inActive">MEDIA INFO</p>
                    </section>
                    <section ref={this.navPages} className="pages">
                        <div className="page hidden select-version">
                            <h3>VERSIONS</h3>
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
                    </section> */}
                </div>
            </div>
        );
    }
}

const mapStateToProps = (state) => ({
    card: state.cardReducer.fetch_card,
    stream: state.videoPlayerReducer
});

const mapActionsToProps = {
    fetchMediaInfo,
    startTranscode
};

export default connect(mapStateToProps, mapActionsToProps)(VideoPlayer);