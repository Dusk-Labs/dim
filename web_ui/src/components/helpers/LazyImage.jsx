import React, { Component } from "react";

class LazyImage extends Component {
    constructor(props){
        super(props)
        this.state = {
            blob: undefined,
            objectUrl: undefined,
        }
    }

    componentDidMount() {
        this.fetchImage()
    }

    componentWillUnmount() {
        if(this.state.objectUrl) {
            URL.revokeObjectURL(this.state.objectUrl)
        }
    }

    fetchImage = async () => {
        const resp = await fetch(this.props.src);
        if (!resp.headers.get("content-type") === "image/jpeg") {
            throw new Error("Not an image");
        }
        const blob = await resp.blob();
        const objectUrl = URL.createObjectURL(blob);
        this.setState({ blob, objectUrl });
        if (this.props.onLoad) {
            await this.props.onLoad(blob)
        }
    }

    render() {
        const { alt } = this.props;
        const { objectUrl } = this.state;

        return objectUrl !== undefined
            ? <div className="image-wrapper"><img src={objectUrl} alt={alt}></img></div>
            : <div className="placeholder"/>;
    }
}

export default LazyImage;