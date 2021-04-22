import React, { PureComponent } from "react";
import { Route, HashRouter, Switch } from "react-router-dom";
import smoothscroll from "smoothscroll-polyfill";
import "./App.css";
import "dhx-suite-package/codebase/suite.css";

import { isEqual } from "lodash";

import Sidebar from "./Sidebar";
import Toolbar from "./Toolbar";
import Dataview from "./Dataview";

class App extends PureComponent {
    constructor(props) {
        super(props);
        smoothscroll.polyfill();
        this.state = {
            toolbarNav: [],
            activeExampleId: "",
        };
    }
    componentDidUpdate() {
        let activeHrefPart = window.location.href.split("/").pop();
        let activeHrefPartCapitalize = activeHrefPart.charAt(0).toUpperCase() + activeHrefPart.slice(1);
        if (this.state.activeWidget !== activeHrefPartCapitalize) {
            this.setState({
                activeWidget: activeHrefPartCapitalize,
            });
        }
    }
    setActiveWidget(activeWidget) {
        this.setState({
            activeWidget: activeWidget.charAt(0).toUpperCase() + activeWidget.slice(1),
        });
        this.el &&
            this.el.scroll({
                top: 0,
                behavior: "smooth",
                inline: "center",
            });
    }
    setToolBarNavItems(array) {
        if (!isEqual(array, this.state.toolbarNav)) {
            this.setState({
                toolbarNav: array,
            });
        }
    }
    setActiveExapmle(id) {
        let elHash = "#" + id;
        const el = this.el.querySelector(elHash);
        const mainY = el.getBoundingClientRect().top + this.el.querySelector("main").scrollTop;
        this.el.querySelector("main").scroll({
            top: mainY - 57,
            behavior: "smooth",
            inline: "center",
        });
    }
    render() {
        return (
            <HashRouter hashType={"slash"}>
                <div
                    className="app-screen"
                    style={{ minHeight: "100vh", maxHeight: "100vh", display: "flex", overflow: "hidden" }}
                >
                    <Sidebar
                        activeWidget={this.state.activeWidget}
                        handleActiveWidgetChange={activeWidget => this.setActiveWidget(activeWidget)}
                    />
                    <div className="app-screen__inner" style={{ width: "calc(100% - 200px)" }}>
                        <Toolbar
                            activeWidget={this.state.activeWidget}
                            activeExampleId={this.state.activeExampleId}
                            scrollToExample={id => this.setActiveExapmle(id)}
                            toolbarNav={this.state.toolbarNav}
                        />
                        <div className="app-content" ref={el => (this.el = el)}>
                            <Switch>
                                <Route
                                    exact
                                    path={`/`}
                                    render={() => (
                                        <Dataview
                                            handleActiveWidgetChange={activeWidget =>
                                                this.setActiveWidget(activeWidget)
                                            }
                                        />
                                    )}
                                />
                            </Switch>
                        </div>
                    </div>
                </div>
            </HashRouter>
        );
    }
}

export default App;