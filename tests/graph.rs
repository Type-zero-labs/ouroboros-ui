//! Smoke / interaction tests for the `graph` layer (egui_kittest).
//!
//! Mirrors `tests/atoms.rs`: render each piece through a harness (fonts installed first) and
//! assert it paints without panicking — which exercises the real layout/paint/borrow paths of the
//! Scene canvas, nodes, handles, edges, controls and minimap. Pointer-level interaction (drag /
//! connect) is validated manually in the storybook; see the spec dev-log.

use egui::{vec2, Ui};
use egui_kittest::Harness;
use ouroboros_ui::graph::{
    EdgeStyle, GraphView, HandleSpec, NodeFrame, NodeId, NodeStatus, Port, PortId, PortSide,
};
use ouroboros_ui::{Mode, Theme};

/// Render `content` with the theme/fonts installed (install frame, then paint frame).
fn rendered(mut content: impl FnMut(&mut Ui) + 'static) {
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        content(ui);
    });
    harness.run();
    harness.run();
}

/// Like [`rendered`] but single-steps — for content that repaints every frame (animated edge).
fn rendered_stepped(mut content: impl FnMut(&mut Ui) + 'static) {
    let mut installed = false;
    let mut harness = Harness::new_ui(move |ui| {
        if !installed {
            Theme::install(ui.ctx(), Mode::Dark);
            installed = true;
            return;
        }
        content(ui);
    });
    harness.step();
    harness.step();
    harness.step();
}

fn out(node: u64) -> Port {
    Port {
        node: NodeId(node),
        port: PortId(1),
        side: PortSide::Out,
    }
}
fn inp(node: u64) -> Port {
    Port {
        node: NodeId(node),
        port: PortId(0),
        side: PortSide::In,
    }
}

#[test]
fn graph_empty_canvas_renders() {
    rendered(|ui| {
        GraphView::new("t_empty")
            .size(vec2(400.0, 300.0))
            .grid(true)
            .show(ui, |_g| {});
    });
}

#[test]
fn graph_nodes_handles_edges_render() {
    rendered(|ui| {
        GraphView::new("t_full")
            .size(vec2(480.0, 320.0))
            .grid(true)
            .controls(true)
            .minimap(true)
            .show(ui, |g| {
                g.node(
                    NodeId(1),
                    egui::pos2(20.0, 30.0),
                    NodeFrame::base().title("A").input(0).output(1),
                    |ui| {
                        ouroboros_ui::atoms::Text::new("body").show(ui);
                    },
                );
                g.node(
                    NodeId(2),
                    egui::pos2(240.0, 30.0),
                    NodeFrame::base().title("B").input(0).output(1),
                    |_ui| {},
                );
                g.edge(out(1), inp(2), EdgeStyle::Default);
                g.edge(out(1), inp(2), EdgeStyle::WithButton);
                g.edge(out(1), inp(2), EdgeStyle::WithLabel);
            });
    });
}

#[test]
fn graph_node_variants_render() {
    rendered(|ui| {
        GraphView::new("t_variants")
            .size(vec2(480.0, 320.0))
            .show(ui, |g| {
                g.node(
                    NodeId(1),
                    egui::pos2(20.0, 20.0),
                    NodeFrame::base()
                        .title("Status")
                        .status(NodeStatus::Running)
                        .appendix("extra info")
                        .tooltip("hover me")
                        .handle(HandleSpec::input(0).label("in"))
                        .handle(HandleSpec::output(1).label("out")),
                    |_ui| {},
                );
                g.node(
                    NodeId(2),
                    egui::pos2(240.0, 20.0),
                    NodeFrame::placeholder().title("Empty"),
                    |_ui| {},
                );
                g.node(
                    NodeId(3),
                    egui::pos2(20.0, 180.0),
                    NodeFrame::base().title("Sized").size(vec2(160.0, 90.0)),
                    |_ui| {},
                );
            });
    });
}

#[test]
fn graph_animated_edge_renders() {
    rendered_stepped(|ui| {
        GraphView::new("t_anim")
            .size(vec2(400.0, 240.0))
            .show(ui, |g| {
                g.node(
                    NodeId(1),
                    egui::pos2(20.0, 40.0),
                    NodeFrame::base().output(1),
                    |_ui| {},
                );
                g.node(
                    NodeId(2),
                    egui::pos2(220.0, 40.0),
                    NodeFrame::base().input(0),
                    |_ui| {},
                );
                g.edge(out(1), inp(2), EdgeStyle::Animated);
            });
    });
}
