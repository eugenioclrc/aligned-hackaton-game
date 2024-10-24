export const Tile = {
    Empty: 0b00,
    Wall: 0b10,
    Box: 0b01,
    Target: 0b11,
    PlayerOnTarget: "PlayeronTarget",
    Player: "p",
    BoxOnTarget: "BoxonTarget",
    fromBits: function(value) {
        switch (value & 0b11) {
            case 0b00: return Tile.Empty;
            case 0b10: return Tile.Wall;
            case 0b01: return Tile.Box;
            case 0b11: return Tile.Target;
            default: return null;
        }
    },
    toBits: function(tile) {
        switch (tile) {
            case Tile.Empty: return 0b00;
            case Tile.Wall: return 0b10;
            case Tile.Box: return 0b01;
            case Tile.Target: return 0b11;
            default: return 0b00;  // Default to empty
        }
    }
};

// Function to convert a hex character to its numeric value
function hexCharToByte(c) {
    if (c >= '0' && c <= '9') return c.charCodeAt(0) - '0'.charCodeAt(0);
    if (c >= 'a' && c <= 'f') return 10 + c.charCodeAt(0) - 'a'.charCodeAt(0);
    if (c >= 'A' && c <= 'F') return 10 + c.charCodeAt(0) - 'A'.charCodeAt(0);
    throw new Error("Invalid hex character");
}

export class Level {
    constructor(
        {row,cols,map,playerRow,playerCol}) {
        this.row = row;
        this.cols = cols;
        this.map = map;
        this.playerRow = playerRow;
        this.playerCol = playerCol;
    }

    // Convert hex string into 2D array of Tiles
    toTileArray() {
        const cleanHex = (this.map || '').startsWith("0x") ? this.map.slice(2) : this.map;
        const requiredHexLength = Math.ceil((this.row * this.cols) / 2);

        if (cleanHex.length < requiredHexLength) {
            throw new Error("Hex string is too short to represent the entire map!");
        }

        const bytes = [];
        for (let i = 0; i < cleanHex.length; i += 2) {
            const high = hexCharToByte(cleanHex[i]) << 4;
            const low = hexCharToByte(cleanHex[i + 1]);
            bytes.push(high | low);
        }

        const tiles = [];
        for (let row = 0; row < this.row; row++) {
            const rowTiles = [];
            for (let col = 0; col < this.cols; col++) {
                const tileIndex = row * this.cols + col;
                const byteIndex = Math.floor(tileIndex / 4);
                const bitOffset = (tileIndex % 4) * 2;
                const byte = bytes[byteIndex];
                const bits = (byte >> (6 - bitOffset)) & 0b11;
                rowTiles.push(Tile.fromBits(bits));
            }
            tiles.push(rowTiles);
        }

        return tiles;
    }

    // Convert 2D array of Tiles into hex map
    static fromTileArray(tiles) {
        const rows = tiles.length;
        const cols = tiles[0].length;
        const totalTiles = rows * cols;

        const byteVec = new Uint8Array(Math.ceil(totalTiles / 4));

        let playerCol = null;
        let playerRow = null;
        for (let row = 0; row < rows; row++) {
            for (let col = 0; col < cols; col++) {
                const tileIndex = row * cols + col;
                const byteIndex = Math.floor(tileIndex / 4);
                const bitOffset = (tileIndex % 4) * 2;

                if(tiles[row][col] == Tile.Player) {
                    playerCol = col;
                    playerRow = row;
                }
                const bits = Tile.toBits(tiles[row][col]);
                byteVec[byteIndex] |= bits << (6 - bitOffset);
            }
        }

        const hexMap = Array.from(byteVec, byte => byte.toString(16).padStart(2, '0')).join('');

        return new Level(rows, cols, hexMap, playerRow, playerCol);
    }
}

// Example usage
const tiles = [
    [Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall],
    [Tile.Wall,  Tile.Empty, Tile.Empty, Tile.Empty, Tile.Empty, Tile.Empty, Tile.Wall],
    [Tile.Wall,  Tile.Player, Tile.Box,   Tile.Empty, Tile.Box,   Tile.Empty, Tile.Wall],
    [Tile.Wall,  Tile.Empty, Tile.Wall,  Tile.Empty, Tile.Wall,  Tile.Empty, Tile.Wall],
    [Tile.Wall,  Tile.Target,Tile.Target,Tile.Empty, Tile.Empty, Tile.Empty, Tile.Wall],
    [Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall,  Tile.Wall]
];
