# Component catalog

60 components across four atomic-design layers, plus the **[graph](#graph)** peer layer
(node editor). Each has its own page with design intent, anatomy, variants/states, API, and
usage examples. Layer rules: atoms paint, everything above composes; the graph layer is the
sanctioned exception that paints (still via tokens). See [guards](../guards.md).

---

## Atoms

23 leaf components — the only layer that paints primitives. Each is a token-driven builder.

### Typography & content
- [Text](./atoms/text.md) — body/label/caption text honoring the full type style
- [Heading](./atoms/heading.md) — display→h-levels titles
- [Icon](./atoms/icon.md) — a Phosphor glyph at a token size
- [Kbd](./atoms/kbd.md) — keyboard key cap

### Actions
- [Button](./atoms/button.md) — the worked-example atom; 6 variants, 3 sizes, icons, loading
- [Toggle](./atoms/toggle.md) — a pressable on/off button

### Form controls
- [Input](./atoms/input.md) — single-line text field
- [Textarea](./atoms/textarea.md) — multi-line text field
- [NumericField](./atoms/numeric_field.md) — numeric input with step
- [Checkbox](./atoms/checkbox.md) — boolean check
- [Radio](./atoms/radio.md) — single-choice dot
- [Switch](./atoms/switch.md) — on/off slider
- [Slider](./atoms/slider.md) — range selector

### Display & feedback
- [Badge](./atoms/badge.md) — status/label pill
- [Avatar](./atoms/avatar.md) — user image/initials
- [Progress](./atoms/progress.md) — determinate progress bar
- [Spinner](./atoms/spinner.md) — indeterminate loading
- [Skeleton](./atoms/skeleton.md) — content placeholder
- [Tooltip](./atoms/tooltip.md) — hover hint

### Structural
- [Surface](./atoms/surface.md) — the painting primitive (fill/border/radius/shadow) everything composes
- [Divider](./atoms/divider.md) — horizontal/vertical rule
- [ColorSwatch](./atoms/color_swatch.md) — a painted color chip
- [SplitterHandle](./atoms/splitter_handle.md) — drag handle for resizable panels

---

## Cells

8 compound row/item building blocks. Compose atoms; never paint.

- [ListItem](./cells/list_item.md) — selectable list row
- [MenuItem](./cells/menu_item.md) — menu row (icon + label + shortcut)
- [PropertyRow](./cells/property_row.md) — inspector row (fixed label column + control)
- [ResponsiveRow](./cells/responsive_row.md) — inspector row that stacks label↔control when narrow
- [TableCell](./cells/table_cell.md) — a single table cell with alignment
- [TableRow](./cells/table_row.md) — a row of table cells
- [ToolbarButton](./cells/toolbar_button.md) — dense toolbar control
- [TreeNode](./cells/tree_node.md) — tree row with expand/select

---

## Molecules

14 compositions of atoms (and smaller molecules).

### Forms
- [Field](./molecules/field.md) — label + control wrapper (+ `FieldGroup`, `FieldSet`, `FieldSeparator`)
- [InputGroup](./molecules/input_group.md) — input with prefix/suffix slots
- [SearchField](./molecules/search_field.md) — input preset for search
- [ColorField](./molecules/color_field.md) — color input field
- [VectorField](./molecules/vector_field.md) — multi-component numeric (x/y/z) field
- [RadioGroup](./molecules/radio_group.md) — grouped radios
- [ToggleGroup](./molecules/toggle_group.md) — segmented toggle buttons
- [CheckboxCard](./molecules/checkbox_card.md) — checkbox as a selectable card
- [RadioCard](./molecules/radio_card.md) — radio as a selectable card

### Containers & navigation
- [Card](./molecules/card.md) — styled surface container
- [Alert](./molecules/alert.md) — inline banner (default/success/warning/error/info)
- [Tabs](./molecules/tabs.md) — tab switcher (default/pill)
- [Breadcrumb](./molecules/breadcrumb.md) — navigation trail
- [Collapsible](./molecules/collapsible.md) — expand/collapse section

---

## Organisms

14 full UI sections composed from cells, molecules, and atoms.

### Layout shells
- [Splitter](./organisms/splitter.md) — the single layout primitive: screen root + resizable panes (`PanelSpec`, with `fixed(px)` non-resizable chrome bands)
- [Panel](./organisms/panel.md) — docked panel chrome: bg + flush edge border + header/footer + padded scroll body (`PanelEdge`)
- [Sidebar](./organisms/sidebar.md) — navigation panel
- [Toolbar](./organisms/toolbar.md) — top/bottom tool bar
- [Menubar](./organisms/menubar.md) — application menu bar

### Overlays
- [Dialog](./organisms/dialog.md) — modal dialog
- [Popover](./organisms/popover.md) — floating anchored surface
- [DropdownMenu](./organisms/dropdown_menu.md) — menu in a popup
- [Toast](./organisms/toast.md) — transient notification
- [Select](./organisms/select.md) — select/combo dropdown

### Data & views
- [Table](./organisms/table.md) — column-defined data table (`Column`, `ColWidth`)
- [TreeView](./organisms/tree_view.md) — hierarchical tree (`TreeItem`)
- [TabView](./organisms/tab_view.md) — tabbed content view
- [Accordion](./organisms/accordion.md) — stacked collapsible sections (`AccordionCtx`)

---

## Graph

The **[graph](./graph/README.md)** peer layer — a reactflow-style node editor on
`egui::Scene`. The one place outside `atoms` that paints (still via tokens). Caller owns the
data; the library owns view-state and reports intents.

- [Graph layer overview](./graph/README.md) — invariant, two tiers, data-model contract, lifecycle
- [identity](./graph/identity.md) — `NodeId`/`PortId`/`NodeKindId`/`PortSide`/`Port`/`Connection`
- [canvas](./graph/canvas.md) — `GraphView`, `GraphCtx`, `GraphResponse`
- [state](./graph/state.md) — `GraphViewState` + drag structs
- [tokens](./graph/tokens.md) — `GraphTokens`
- [node](./graph/node.md) — `NodeFrame`/`NodeResult`/`NodeStatus` + `ctx.node`
- [edge](./graph/edge.md) — `EdgeStyle`/`EdgeResult` + `ctx.edge`
- [handle](./graph/handle.md) — `HandleSpec`/`HandleVariant` (ports)
- [search](./graph/search.md) — `NodeSearch` palette
- [viewport](./graph/viewport.md) — standalone world↔screen transform helper
- [extras](./graph/extras.md) — `grid`, `resizer`, `minimap`, `toolbar`, `controls`

---

## Page template

Every component page follows the same structure: **what it is** → **Design** (purpose,
anatomy, variants/sizes/states, tokens consumed, a11y) → **API** (builder methods) →
**Usage** (minimal + realistic examples) → **Composition** → **Notes**.
</content>
