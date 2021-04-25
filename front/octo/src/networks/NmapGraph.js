import React, { Component } from 'react';

import { Diagram as DiagramDHX } from 'dhx-diagram-package';

import 'dhx-diagram-package/codebase/diagram.css'
import "dhx-suite-package/codebase/suite.css";

import './NmapGraph.css';

class Diagram extends Component {
    componentDidMount() {
        var diag = new DiagramDHX("diagram_container", {
            type: "org",
            defaults: {
                template: {
                    height: 115,
                    width: 330
                }
            }
        });

        diag.addShape("networkCard", {
            template: config => (
                `<section class='template'>
                    <img src='${config.img}' alt='${config.text}'></img>
                    <span>${config.text}</span>
                    <span>${config.ip}</span>
                </section>`
            ),
            defaults: {
                width: 160,
                height: 160,
            }
        });

        var data = [
            {
                "id": "1",
                "type": "networkCard",
                "ip": "192.168.1.1",
                "text": "MYHOSTNAME",
                "img": "/server.svg",
            },
            {
                "id": "2",
                "type": "networkCard",
                "ip": "192.168.1.10",
                "text": "MYHOSTNAME",
                "img": "/desktop.svg",

                "parent": "1"
            },
            {
                "id": "3",
                "type": "networkCard",
                "ip": "192.168.1.23",
                "text": "MYHOSTNAME",
                "img": "/desktop.svg",

                "parent": "1"
            },
            {
                "id": "4",
                "type": "networkCard",
                "ip": "192.168.1.27",
                "text": "MYHOSTNAME",
                "img": "/desktop.svg",

                "parent": "1"
            }
        ];
        diag.data.parse(data);
    }

    render() {
        return (
            <div className="dhx_sample-container__widget" id="diagram_container"></div>
        );
    }
}

class NmapGraph extends Component {
    render() {
        return (
            <section className="dhx_sample-container">
                <Diagram />
            </section>
        );
    }
}

export default NmapGraph;