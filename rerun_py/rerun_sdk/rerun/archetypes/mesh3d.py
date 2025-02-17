# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/mesh3d.fbs".

# You can extend this class by creating a "Mesh3DExt" class in "mesh3d_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from .mesh3d_ext import Mesh3DExt

__all__ = ["Mesh3D"]


@define(str=False, repr=False, init=False)
class Mesh3D(Mesh3DExt, Archetype):
    """
    **Archetype**: A 3D triangle mesh as specified by its per-mesh and per-vertex properties.

    See also [`Asset3D`][rerun.archetypes.Asset3D].

    Example
    -------
    ### Simple indexed 3D mesh:
    ```python
    import rerun as rr

    rr.init("rerun_example_mesh3d_indexed", spawn=True)

    rr.log(
        "triangle",
        rr.Mesh3D(
            vertex_positions=[[0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            vertex_normals=[0.0, 0.0, 1.0],
            vertex_colors=[[0, 0, 255], [0, 255, 0], [255, 0, 0]],
            indices=[2, 1, 0],
            mesh_material=rr.Material(albedo_factor=[0xCC, 0x00, 0xCC, 0xFF]),
        ),
    )
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/mesh3d_simple/e1e5fd97265daf0d0bc7b782d862f19086fd6975/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/mesh3d_simple/e1e5fd97265daf0d0bc7b782d862f19086fd6975/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/mesh3d_simple/e1e5fd97265daf0d0bc7b782d862f19086fd6975/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/mesh3d_simple/e1e5fd97265daf0d0bc7b782d862f19086fd6975/1200w.png">
      <img src="https://static.rerun.io/mesh3d_simple/e1e5fd97265daf0d0bc7b782d862f19086fd6975/full.png" width="640">
    </picture>
    </center>
    """

    # __init__ can be found in mesh3d_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            vertex_positions=None,  # type: ignore[arg-type]
            mesh_properties=None,  # type: ignore[arg-type]
            vertex_normals=None,  # type: ignore[arg-type]
            vertex_colors=None,  # type: ignore[arg-type]
            mesh_material=None,  # type: ignore[arg-type]
            class_ids=None,  # type: ignore[arg-type]
            instance_keys=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> Mesh3D:
        """Produce an empty Mesh3D, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    vertex_positions: components.Position3DBatch = field(
        metadata={"component": "required"},
        converter=components.Position3DBatch._required,  # type: ignore[misc]
    )
    # The positions of each vertex.
    #
    # If no `indices` are specified, then each triplet of positions is interpreted as a triangle.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    mesh_properties: components.MeshPropertiesBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MeshPropertiesBatch._optional,  # type: ignore[misc]
    )
    # Optional properties for the mesh as a whole (including indexed drawing).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    vertex_normals: components.Vector3DBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.Vector3DBatch._optional,  # type: ignore[misc]
    )
    # An optional normal for each vertex.
    #
    # If specified, this must have as many elements as `vertex_positions`.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    vertex_colors: components.ColorBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColorBatch._optional,  # type: ignore[misc]
    )
    # An optional color for each vertex.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    mesh_material: components.MaterialBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MaterialBatch._optional,  # type: ignore[misc]
    )
    # Optional material properties for the mesh as a whole.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    class_ids: components.ClassIdBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ClassIdBatch._optional,  # type: ignore[misc]
    )
    # Optional class Ids for the vertices.
    #
    # The class ID provides colors and labels if not specified explicitly.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    instance_keys: components.InstanceKeyBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.InstanceKeyBatch._optional,  # type: ignore[misc]
    )
    # Unique identifiers for each individual vertex in the mesh.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
