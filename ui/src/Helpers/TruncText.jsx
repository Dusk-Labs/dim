import { Component, Fragment } from "react";

class TruncText extends Component {
  render() {
    const words = this.props.content.split(" ");

    if (words.length < this.props.max) {
      return <Fragment>{words.join(" ")}</Fragment>
    }

    words.length = this.props.max;
    return <Fragment>{words.join(" ") + "..."}</Fragment>
  }
}

export default TruncText;