import React, { Component } from "react";

class TruncText extends Component {
    render() {
        const words = this.props.content.split(" ");

        if (words.length < this.props.max) {
            return <p>{words.join(" ")}</p>
        }

        words.length = this.props.max;
        return <p>{words.join(" ") + "..."}</p>
    }
}

export default TruncText;