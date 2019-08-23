import React, { Component } from "react";
import "./BannerPage.scss";

class BannerPage extends Component {
    constructor(props) {
        super(props);

        this.state = {
            activeIndex: 0,
            interval: 14000
        }

        this.interval = setInterval(this.next.bind(this), this.state.interval);
    }

    next = async () => {
        let { length } = this.props.children;
        const index = this.state.activeIndex;
        const nextIndex = index < --length ? index + 1 : 0

        this.setState({
            activeIndex: nextIndex,
        });
    }

    toggle = async (e) => {
        clearInterval(this.interval);

        this.setState({
            activeIndex: parseInt(e.currentTarget.dataset.key)
        });

        this.interval = setInterval(this.next.bind(this), this.state.interval);
    }

    componentWillUnmount() {
        clearInterval(this.interval);
    }

    render() {
        const { activeIndex } = this.state;
        const banners = [];
        const crumbs = [];

        // eslint-disable-next-line
        for (const child in this.props.children) {
            const active = activeIndex === parseInt(child) ? "active" : "hidden";

            banners.push(
                <div className={active} key={child}>
                    {this.props.children[child]}
                </div>
            );

            crumbs.push(
                <span className={active} key={child} data-key={child} onClick={this.toggle}></span>
            );
        };

        return (
            <div className="banner-wrapper">
                <div className="pages">{banners}</div>
                <div className="crumbs">{crumbs}</div>
            </div>
        );
    }
}

export default BannerPage;
