import React, { Component } from "react";
import "./progress-bar.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faCircle } from "@fortawesome/free-solid-svg-icons";

library.add(faCircle);

class ProgressBar extends Component {
    constructor(props) {
        super(props);

        this.state = {
            data: {}
        };
    }

    componentDidMount() {
        // fetch(`api/${this.props.id}`)
        //     .then(res => res.json())
        //     .then(({data}) => {
        //         this.setState({
        //             data: {
        //                 current: ,
        //                 currentUnit: ,
        //                 duration: ,
        //                 durationUnit: ,
        //                 season: ,
        //                 episode:
        //             }
        //         })
        //     });
        this.setState({
            data: {
                current: 23,
                currentUnit: "min",
                duration: 53,
                durationUnit: "min",
                season: "01",
                episode: "01"
            }
        })
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

        return (
            <div className="progress-bar">
                <div className="s-e">
                    <p>S{season}</p>
                    <FontAwesomeIcon icon="circle"/>
                    <p>E{episode}</p>
                </div>
                <div className="progress">
                    <div className="current">
                        <p>{current}</p>
                        <p>{currentUnit}</p>
                    </div>
                    <div className="bar">
                        <progress max={duration} value={current}></progress>
                    </div>
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
