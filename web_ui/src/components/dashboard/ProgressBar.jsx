import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import "./ProgressBar.scss";

class ProgressBar extends Component {
    constructor(props) {
        super(props);

        this.state = {
            data: {}
        };
    }

    componentDidMount() {
        this.setState({
            data: {
                current: Math.round(this.props.delta / 60),
                duration: Math.round(this.props.duration / 60),
                season: this.props.season,
                episode: this.props.episode,
            },
            accent: this.props.accent,
        })
    }

    static getDerivedStateFromProps(nextProps) {
        return {
            accent: nextProps.accent,
            data: {
                current: Math.round(nextProps.delta / 60),
                duration: Math.round(nextProps.duration / 60),
                season: nextProps.season,
                episode: nextProps.episode,
            },
        };
    }

    render() {
        const {
            current,
            duration,
            season,
            episode
        } = this.state.data;

        const { accent } = this.state;
        const width = current / duration * 100 + "%";

        return (
            <div className="progress-bar">
                { (season, episode !== undefined) &&
                    (<div className="s-e">
                        <p>S{season}</p>
                        <FontAwesomeIcon icon="circle" style={{ color: accent }}/>
                        <p>E{episode}</p>
                    </div>)
                }
                <div className="progress">
                    <div className="current">
                        <p>{current | "N/A"}</p>
                        <p>min</p>
                    </div>
                    <span className="bar">
                        <span className="progress" style={{ width: width, background: accent }}></span>
                    </span>
                    <div className="duration">
                        <p>{duration | "N/A"}</p>
                        <p>min</p>
                    </div>
                </div>
            </div>
        );
    }
}

export default ProgressBar;
