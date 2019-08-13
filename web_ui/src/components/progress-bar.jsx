import React, { Component } from "react";
import "./progress-bar.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

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
                current: 23,
                currentUnit: "min",
                duration: 53,
                durationUnit: "min",
                season: "01",
                episode: "01",
            },
            accent: this.props.accent,
        })
    }

    static getDerivedStateFromProps(nextProps, prevState) {
        return {
            accent: nextProps.accent,
        };
    }

    render() {
        const {
            current,
            currentUnit,
            duration,
            durationUnit,
            season,
            episode
        } = this.state.data;

        const { accent } = this.state;

        const width = current / duration * 100 + "%";

        return (
            <div className="progress-bar">
                <div className="s-e">
                    <p>S{season}</p>
                    <FontAwesomeIcon icon="circle" style={{ color: accent }}/>
                    <p>E{episode}</p>
                </div>
                <div className="progress">
                    <div className="current">
                        <p>{current}</p>
                        <p>{currentUnit}</p>
                    </div>
                    <span className="bar">
                        <span className="progress" style={{ width: width, background: accent }}></span>
                    </span>
                    <div className="duration">
                        <p>{duration}</p>
                        <p>{durationUnit}</p>
                    </div>
                </div>
            </div>
        );
    }
}

export default ProgressBar;
