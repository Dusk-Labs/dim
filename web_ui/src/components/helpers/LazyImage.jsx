import React, { Component } from "react";

class LazyImage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            acquired: false,
            blob: null,
        };

        fetch(this.props.src)
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
                }
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
