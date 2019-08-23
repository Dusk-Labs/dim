import React, { Component } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { NavLink } from "react-router-dom";

class FloatingBar extends Component {

    constructor(props) {
        super(props);

        this.bar = React.createRef();

        this.state = {
            lastScrollY: 0
        }
    }

    componentDidMount() {
        window.addEventListener("scroll", this.handleScroll, true);
        window.addEventListener("mousemove", this.handleMouse, true);
    }

    componentWillUnmount() {
        window.removeEventListener("scroll", this.handleScroll);
        window.removeEventListener("mousemove", this.handleMouse);
    }

    handleScroll = () => {
        if (window.scrollY % 50 !== 0 || !this.bar.current) return;

        window.scrollY > this.state.lastScrollY
            ? this.bar.current.style.bottom = `-6%`
            : this.bar.current.style.bottom = `1%`;

        this.setState({
            lastScrollY: window.scrollY
        })
    };

    render() {
        return (
            <div className="floating-bar" ref={this.bar}>
                <NavLink to="/">
                    <FontAwesomeIcon icon="home"/>
                </NavLink>
            </div>
        )
    }
}

export default FloatingBar;