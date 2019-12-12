import React, { Component } from "react";
import LazyImage from "../helpers/LazyImage.jsx";

class MediaPage extends Component {
    constructor(props) {
        super(props);
        this.state = {
            mediaid: this.props.match.params.id,
            data: {},
        }
    }

    async componentDidMount() {
        await this.getData();
    }

    async getData() {
        const { mediaid } = this.state;
        const extra_info = await (await fetch(`http://${window.host}:8000/api/v1/media/${mediaid}/info`)).json();
        let info = await (await fetch(`http://${window.host}:8000/api/v1/media/${mediaid}`)).json();

        if(info.media_type === "tv") {
            const seasons = await (await fetch(`http://${window.host}:8000/api/v1/tv/${mediaid}/season`)).json();
            const full_data = seasons.map(async x => { return {...x, episodes: (await (await fetch(`http://${window.host}:8000/api/v1/tv/${mediaid}/season/${x.season_number}/episode`)).json())}});
            info = {...info, seasons: full_data}
        }

        await this.setState({data: {...info, extra_info}});
    }

    render() {
        const { backdrop_path, description, duration, genres, name, poster_path, rating, year } = this.state.data;
        return (
            <div>
                <LazyImage src={backdrop_path} />
                <LazyImage src={poster_path} />
                <div className="name">{name}</div>
                <div className="description">{description}</div>
                <div className="duration">{duration}</div>
                <div className="rating">{rating}</div>
                <div className="year">{year}</div>
                {genres !== undefined ? genres.map(x => <div key={x} className="genre">{x}</div>) : <div/>}
            </div>
        );
    }
}

export default MediaPage;
