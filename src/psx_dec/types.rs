/* Let's recreate the whell again */
type CLUTCoordI = u8;
type CLUTI = u8;
type EntitySpritePartI = u16;
type EntityGfxI = u8;
type EntityLayoutI = u8;
type MapLayoutI = u8;
type TileI = u16;

struct C2DPos_t<T> {
    x: T,
    y: T
}

type C2DPosBig = C2DPos_t<u16>; // 2d_bigpos
type C2DPos = C2DPos_t<u8>; // 2d_pos, 2d_smallpos_16, 2d_smallpos_256

struct C2DSize_t<T> {
    w: T,
    h: T
}

type C2DSizeBig = C2DSize_t<u16>; // 2d_bigsize
type C2DSize = C2DSize_t<u8>; // 2d_size

struct C2DResize {
    src: C2DSize,
    dst: C2DSize
} // 2d_resize

struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
} // rgba

struct PsxPtr<T> {
    orig: u32,
    load: Box<*mut T>
} // psx_ptr

type CLUT = Vec<RGBA>; // clut

enum CollisionType {
} // collision_type

struct EntitySpritePart {
    draw_flags: u16,
    dst_pos: C2DPosBig,
    dst_dim: C2DSizeBig,
    clut_r: u16,
    texture_pg: u16,
    src_begin_pos: C2DPosBig,
    src_end_pos: C2DPosBig,
}

struct EntityCodeSprite {
    is_mapped: bool,
    count: Option<u16>,
    index: Option<EntitySpritePartI>,
    parts: Option<Vec<EntitySpritePart>>,
    dst: Option<C2DPosBig>
} // entity_code_sprite, entity_sprite_partref

type EntityCodeState = u16; // entity_code_state
type EntityAIState = u16; // entity_ai_state

struct EntityIdentity {
    type_id: u16
} // entity_identity

struct EntityState {
    death_id: u8,
    type_id: u8
} // entity_state

struct EntityDef {
    pos: C2DPosBig,
    identity: EntityIdentity,
    state: EntityState,
    ai_state: EntityAIState
} // entity_def

struct EntityGfxEntry {
    vram_pos: C2DPos,
    dim: C2DSize,
    unc_data: Vec<u8>
} // entity_gfxentry

type EntityGfxPkg = Vec<EntityGfxEntry>; // entity_gfxpkg
type EntityGfxDef = Vec<EntityGfxPkg>; // entity_gfxdef

type MapInternPtr<T> = PsxPtr<T>; // map_intern_ptr

type MapSection = u8;
type MapHeader = Vec<MapInternPtr<MapSection>>; // map_header

struct Map {
    sections: MapHeader,
    data: Vec<u8>
} // map

struct TileDef {
    tileset_coords: PsxPtr<Vec<C2DPos>>,
    tile_coords: PsxPtr<Vec<C2DPos>>,
    clut_coords: PsxPtr<Vec<C2DPos>>,
    collisions: PsxPtr<Vec<CollisionType>>
} // tiledef

struct MapLayer {
    tiles: MapInternPtr<Vec<TileI>>,
    tiles_def: MapInternPtr<TileDef>,
    dim: C2DResize,
    tile_flags: u8,
    z_index: u8,
    z_priority: u8,
    layer_flags: u16
} // map_layer

struct MapLayout {
    foreground: MapInternPtr<MapLayer>,
    background: MapInternPtr<MapLayer>
} // map_layout

type MapLsEntity = Vec<EntityDef>; // map_lsentity
type MapLsLayout = Vec<MapLayout>; // map_lslayout

struct MapRoom {
    start: C2DPos,
    end: C2DPos,
    layout: MapLayoutI,
    ent_gfx: EntityGfxI,
    ent_layout: EntityLayoutI
} // map_room

type MapLsRoom = Vec<MapRoom>; // map_lsroom

type TilesetTile = Vec<CLUT>; // tileset_tile
type TilesetGroup = Vec<TilesetTile>; // tileset_group
type TilesetSlot = Vec<TilesetGroup>; // tileset_slot
type Tileset = Vec<TilesetSlot>; // tileset
