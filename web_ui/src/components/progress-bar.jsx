import React, { Component } from "react";
import "./progress-bar.scss";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faCircle } from "@fortawesome/free-solid-svg-icons";

library.add(faCircle);

class ProgressBar extends Component {
    render() {
        return (
            <div className="progress-bar">
                <div className="s-e">
                    <p>S01</p>
                    <FontAwesomeIcon icon="circle"/>
                    <p>E01</p>
                </div>
                <div className="progress">
                    <div className="current">
                        <p>23</p>
                        <p>min</p>
                    </div>
                    <div className="bar">
                        <progress max="53" value="23"></progress>
                    </div>
                    <div className="duration">
                        <p>23</p>
                        <p>min</p>
                    </div>
                </div>
            </div>
        );
    }
}

export default ProgressBar;
