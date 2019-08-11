import React, { Component } from "react";

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            acquired: false,
            blob: null,
            src: this.props.src,
        };
        this.fetchImage();
    }

    fetchImage() {
        fetch(this.state.src)
            .then(resp => {
                if (resp.headers.get("content-type") === "image/jpeg") {
                    return resp.blob();
                }
            })
            .then((blob) => {
                if (blob) {
                    const blobUrl = URL.createObjectURL(blob);

                    this.setState({
                        acquired: true,
                        blob: blobUrl
                    });
                    return blob;
                }
            })
            .then((blob) => {
                this.props.callback(blob);
            });
    }

    render() {
        const { acquired, blob } = this.state;

        return acquired
            ? <div className="image-wrapper"><img src={blob} alt={blob}></img></div>
            : <div className="placeholder"></div>;
    }
}

export default LazyImage;
