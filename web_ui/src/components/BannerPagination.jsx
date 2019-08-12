import React, { Component } from "react";
import "./banner-pages.scss";

import { library } from "@fortawesome/fontawesome-svg-core";
import { faArrowAltCircleRight } from "@fortawesome/free-solid-svg-icons";

library.add(faArrowAltCircleRight);

class BannerPages extends Component {
    constructor(props) {
        super(props);
        this.state = {
            activeIndex: 0,
        }

        setInterval(this.next.bind(this), 14000);
    }

    next = async () => {
        const children = this.props.children;
        const activeIndex = this.state.activeIndex;
        const nextNum = activeIndex < children.length - 1 ? activeIndex + 1 : 0

        this.setState({
            activeIndex: nextNum,
        });
    }

    toggle = async (e) => {
        this.setState({
            activeIndex: parseInt(e.currentTarget.dataset.key)
        });
    }

    render() {
        const { activeIndex } = this.state;
        const children = [];
        const crumbs = [];

        for (var child in this.props.children) {
            const active = activeIndex === parseInt(child) ? "active" : "hidden";
            children.push(
                <div className={"page " + active} key={child}>
                    { this.props.children[child] }
                </div>
            );

            crumbs.push(
                <div className={"crumb " + active} key={child} data-key={child} onClick={this.toggle}></div>
            );
        };

        return (
            <div>
                <div className="pages">
                    { children }
                </div>
                <div className="crumbs">
                    { crumbs }
                </div>
            </div>
        );
    }
}

export default BannerPages;
