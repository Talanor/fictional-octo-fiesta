import React, { PureComponent } from "react";
import { Sidebar as SidebarDHX } from "dhx-suite-package";
import { withRouter } from "react-router-dom";

import NmapGraph from "./networks/NmapGraph";

class Sidebar extends PureComponent {
    componentDidUpdate() {
        this.sidebar.data.map(item => (item.active = false));
        const activeWidget = window.location.href.split("/").pop();
        this.sidebar.data.update(activeWidget + "-link", { active: true });
    }
    componentDidMount() {
        this.sidebar = new SidebarDHX(this.el, {
            css: "dhx_widget--border_right",
            data: [
                {
                    id: "logo",
                    type: "customButton",
                    css: "logo-button",
                    html: `<img src="${process.env.PUBLIC_URL}/octopus.svg" alt="Octo Fiesta Logo"/>`,
                    group: "nav",
                    twoState: true,
                },
                {
                    value: "Network",
                    id: "networks-link",
                    group: "nav",
                    twoState: true,
                },
                {
                    value: "Notifications",
                    id: "notifications-link",
                    group: "nav",
                    twoState: true,
                },
                {
                    value: "Settings",
                    id: "settings-link",
                    group: "nav",
                    twoState: true,
                },
            ],
        });
        const activeWidget = window.location.href.split("/").pop();
        if (activeWidget) {
            this.props.handleActiveWidgetChange(activeWidget);
        }
        this.sidebar.events.on("click", id => {
            if (id !== "logo") {
                const widgetName = id.split("-")[0];
                this.props.history.push("/" + widgetName);
                this.props.handleActiveWidgetChange(widgetName);
                if (activeWidget) {
                    this.sidebar.data.update(activeWidget + "-link", { active: false });
                }
                this.sidebar.data.update(widgetName + "-link", { active: true });
            } else {
                this.props.history.push("/");
                this.props.handleActiveWidgetChange("");
                if (activeWidget) {
                    this.sidebar.data.update(activeWidget + "-link", { active: false });
                }
            }
        });
    }
    componentWillUnmount() {
        this.sidebar && this.sidebar.destructor();
    }
    render() {
        return <div style={{ maxHeight: "100vh" }} ref={el => (this.el = el)}></div>;
    }
}

export default withRouter(Sidebar);