import React, { Component } from "react";
import { connect } from "react-redux";
import * as Vibrant from "node-vibrant";
import videojs from "video.js";
import "videojs-contrib-dash";
import "dashjs";

import { fetchMediaInfo, fetchExtraMediaInfo } from "../actions/cardActions.js";

import WithOutSidebarLayout from "../Layouts/WithOutSidebarLayout.jsx";
import VideoPlayerControls from "./VideoPlayerControls.jsx";
import LazyImage from "../Helpers/LazyImage.jsx";

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

        this.triggerUserActive = this.triggerUserActive.bind(this);
        this.handleVideoLoaded = this.handleVideoLoaded.bind(this);
        this.hardSkip = this.hardSkip.bind(this);

        this.state = {
            userActiveTimeout: null,
            navIndex: -1
        };
    }

    componentDidMount() {
        document.title = "Dim - Playing";

        document.querySelector("meta[name='theme-color']").setAttribute("content", "#000000");

        document.addEventListener("mousemove", this.triggerUserActive);
        document.addEventListener("scroll", this.triggerUserActive);
        document.addEventListener("keydown", this.triggerUserActive);
        document.addEventListener("resize", this.triggerUserActive);

        this.video.current.addEventListener("loadeddata", this.handleVideoLoaded);

        const { id } = this.props.match.params;

        this.props.fetchMediaInfo(this.props.auth.token, id);
        this.props.fetchExtraMediaInfo(this.props.auth.token, id);
    }

    componentWillUnmount() {
        document.querySelector("meta[name='theme-color']").setAttribute("content", "#333333");

        document.removeEventListener("mousemove", this.triggerUserActive);
        document.removeEventListener("scroll", this.triggerUserActive);
        document.removeEventListener("keydown", this.triggerUserActive);
        document.removeEventListener("resize", this.triggerUserActive);

        this.video.current.removeEventListener("loadeddata", this.handleVideoLoaded);

        clearTimeout(this.state.userActiveTimeout);
        this.body.style.cursor = "default";

        if (typeof this.player !== "undefined")
            this.player.dispose();
    }

    componentDidUpdate(prevProps) {
        // FETCH_MEDIA_INFO_OK
        if (
            prevProps.media_info.fetched !== this.props.media_info.fetched
            && !this.props.media_info.error
            && !this.state.endsAt
        ) {
            const currentDate = new Date();
            const { duration } = this.props.media_info.info;

            currentDate.setSeconds(currentDate.getSeconds() + duration);

            const endsAt = currentDate.toLocaleString("en-US", {
                hour: "numeric",
                minute: "numeric",
                hour12: true
            });

            this.setState({endsAt});
        }

        // FETCH_MEDIA_EXTRA_INFO_OK
        if (prevProps.extra_media_info.fetched !== this.props.extra_media_info.fetched && !this.props.extra_media_info.error) {
            if (this.props.extra_media_info.info.versions) {
                const { id } = this.props.extra_media_info.info.versions[0];

                this.player = videojs(this.video.current);

                this.player.ready(_ => {
                    this.player.src({
                        src: `//${window.host}:8000/api/v1/stream/${id}/manifest.mpd`,
                        type: "application/dash+xml"
                    });
                });
            };
            window.player = this.player;
        }
    }

    updateEndsAt = (endsAt) => this.setState({endsAt});

    hardSkip(skipTo) {
        this.player.currentTime(skipTo);
    }

    handleVideoLoaded() {
        this.video.current.play();
        this.triggerUserActive();
    }

    triggerUserActive(e) {
        if (e?.type === "mousemove") {
            if (e.x % 15 !== 0) return;
        }

        if (this.overlay.current) {
            this.overlay.current.style.opacity = 1;
        }

        if (this.state.userActiveTimeout) {
            this.body.style.cursor = "unset";
            clearTimeout(this.state.userActiveTimeout);
        }

        if (this.video.current?.readyState === 4 && !this.video.current?.paused) {
            const userActiveTimeout = setTimeout(_ => {
                if (this.overlay.current) {
                    this.overlay.current.style.opacity = 0;
                }

                this.body.style.cursor = "none";
            }, 3000);

            this.setState({userActiveTimeout});
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

    //     for (let [i, navLink] of [...this.navLinks.current.children].entries()) {
    //         if (i === index) {
    //             navLink.classList.add("active");
    //             navLink.classList.remove("inActive");
    //             continue;
    //         };

    //         navLink.classList.add("inActive");
    //         navLink.classList.remove("active");
    //     }

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

        // FETCH_MEDIA_INFO_OK
        if (this.props.media_info.fetched && !this.props.media_info.error) {
            const { name, poster_path } = this.props.media_info.info;

            posterPath = poster_path;
            document.title = `Dim - Playing '${name}'`;
        }

        const videoPlayer = (
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
                        // NOTE: Unsure if passing a direct callback to a child is a good idea pattern wise.
                        <VideoPlayerControls video={this.video.current} card={this.props.media_info} hardSkip={this.hardSkip}/>
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

        return (
            <WithOutSidebarLayout>
                {videoPlayer}
            </WithOutSidebarLayout>
        )
    }
}

const mapStateToProps = (state) => ({
    auth: state.authReducer,
    media_info: state.cardReducer.media_info,
    extra_media_info: state.cardReducer.extra_media_info
});

const mapActionsToProps = {
    fetchMediaInfo,
    fetchExtraMediaInfo
};

export default connect(mapStateToProps, mapActionsToProps)(VideoPlayer);
