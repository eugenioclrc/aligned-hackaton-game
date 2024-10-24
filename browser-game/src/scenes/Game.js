import { Scene } from 'phaser';

const gameData = {"row":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","playerRow":2,"playerCol":1}

import { Tile, Level } from '../lib/lib.js';


export class Game extends Scene
{
    
    constructor ()
    {
        super('Game');
    }

    preload ()
    {
        this.load.image('tiles', 'assets/sokoban_tilesheet@2.png');
        this.load.spritesheet('player', 'assets/sokoban_tilesheet@2.png', { frameWidth: 128, frameHeight: 128 });

    }

    create () {
        this.playableMap = (new Level(gameData)).toTileArray();
        let _map = this.playableMap;

        this.cameras.main.setBackgroundColor(0x000);

        //this.add.image(512, 384, 'background').setAlpha(0.5);

        this.add.text(512, 384, 'Make something fun!\nand share it with us:\nsupport@phaser.io', {
            fontFamily: 'Arial Black', fontSize: 38, color: '#ffffff',
            stroke: '#000000', strokeThickness: 8,
            align: 'center'
        }).setOrigin(0.5);

        // Creating a blank tilemap with the specified dimensions
        this.map = this.make.tilemap({ tileWidth: 128, tileHeight: 128, width: _map[0].length, height: _map.length  });
        const tiles = this.map.addTilesetImage('tiles');

        const layerBg = this.map.createBlankLayer('layerBg', tiles);

        // Add a simple scene with some random element
        layerBg.fill(89, 0, 0, _map[0].length, _map.length); // Surface of the water
        layerBg.setScale(0.5);

        const layer = this.map.createBlankLayer('layer1', tiles);

        // Add a simple scene with some random element
        // layer.fill(58, 0, 13, 25, 1); // Surface of the water
        // layer.fill(77, 0, 14, 25, 5); // Body of the water
        //layer.randomize(0, 0, 25, 13, [ 44, 45, 46, 47, 48 ]); // Wall above the water
        layer.setScale(0.5);

        this.boxes = []
        this.boxTarget = [];

        _map.forEach((row, y) => {
            row.forEach((tile, x) => {
                if(tile === Tile.Wall) {
                    layer.putTileAt(85, x, y);
                } else if(tile === Tile.Box) {
                    const b = this.add.sprite(x * 64 + 32,y * 64 + 32, 'player', 1);
                    b.nameCords = `${y}-${x}`;
                    this.boxes.push( b.setScale(0.5));
                    //layer.putTileAt(1, x, y);
                } else if(tile === Tile.Target) {
                    layer.putTileAt(40, x, y);
                    this.boxTarget.push(`${y}-${x}`);
                } else if(tile === Tile.Empty) {

                }
            });
        });


        // Player animation
        this.anims.create({
            key: 'idle',
            frames: this.anims.generateFrameNumbers('player', { start: 52, end: 52 }),
            frameRate: 10,
            repeat: -1
        });

        // Adding the player sprite at the initial position from gameData
        this.player = this.add.sprite(gameData.playerCol * 64 + 32, gameData.playerRow * 64 + 32, 'player').setScale(0.5);
        this.player.play('idle');


        // Position tracking
        this.playerRow = gameData.playerRow;
        this.playerCol = gameData.playerCol;

        //

        // Keyboard input
        this.cursors = this.input.keyboard.createCursorKeys();
        this.cursors.S = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.S);
        this.cursors.W = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.W);
        this.cursors.A = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.A);
        this.cursors.D = this.input.keyboard.addKey(Phaser.Input.Keyboard.KeyCodes.D);


        // Position tracking
        this.playerRow = gameData.playerRow;
        this.playerCol = gameData.playerCol;

        this.movements = [{
            playerCol: this.playerCol,
            playerRow: this.playerRow,
            playableMap: this.playableMap,
            boxes: this.boxes.map((b, i) => {
                return {
                    position: i,
                    x: b.x,
                    y: b.y,
                    nameCords: b.nameCords
                }
            })
        }];
   
        /*
        this.input.once('pointerdown', () => {

            this.scene.start('GameOver');

        });
        */

        this.cameras.main.setBounds(this.map.x, this.map.y, this.map.widthInPixels * 2, this.map.heightInPixels * 2);
        
        //this.cameras.main.startFollow(this.player, true);

        this.input.keyboard.on('keydown-R', event =>
            {
                if (confirm("Are you sure you want to restart the level?")) {
                    
                    this.scene.start('Game');
                }

            });


        this.input.keyboard.on('keydown-BACKSPACE', event =>
            {
                this.undoMovement();
            });

    }

    update () {
        if (this.isMoving) { // Only allow movement when not moving
            return;
        }

        // Player movement handling
        if (this.cursors.left.isDown || this.cursors.A.isDown) {
            this.movePlayer('left');
        }
        else if (this.cursors.right.isDown  || this.cursors.D.isDown) {
            this.movePlayer('right');
        }
        else if (this.cursors.up.isDown || this.cursors.W.isDown) {
            this.movePlayer('up');
        }
        else if (this.cursors.down.isDown || this.cursors.S.isDown) {
            this.movePlayer('down');
        }
    }

    undoMovement() {
        if(this.movements.length < 2 || this.isMoving) {
            return;
        }
        this.isMoving = true;

        const lastMovement = this.movements[this.movements.length - 1];
        this.movements.pop()

        this.playerCol = lastMovement.playerCol;
        this.playerRow = lastMovement.playerRow;

        this.playableMap = lastMovement.playableMap;
        lastMovement.boxes.forEach(bm => {
            this.boxes[bm.position].x = bm.x;
            this.boxes[bm.position].y = bm.y;
            this.boxes[bm.position].nameCords = bm.nameCords;
        });
        
        this.tweens.add({
            targets: this.player,
            x: lastMovement.playerCol * 64 + 32,
            y: lastMovement.playerRow * 64 + 32,
            duration: 300, // Animation duration in ms
            onComplete: () => {
                this.isMoving = false; // Re-enable input when animation ends
            }
        });

        
    }

    movePlayer(dir) {
        let deltaX = 0;
        let deltaY = 0;
        if(dir === 'left') {
            deltaX= -1;
        } else if(dir === 'right') {
            deltaX= 1;
        } else if(dir === 'up') {
            deltaY= -1;
        } else if(dir === 'down') {
            deltaY= 1;
        } else {
            throw new Error('Invalid direction');
        }

        let _map = this.playableMap;
        const newRow = this.playerRow + deltaY;
        const newCol = this.playerCol + deltaX;

        
        // Check if the new position is valid
        if (_map[newRow][newCol] === Tile.Empty || _map[newRow][newCol] === Tile.Target || _map[newRow][newCol] === Tile.Box
            || _map[newRow][newCol] === Tile.BoxOnTarget
        ) {
            // if push a box is necessary to check if box can be pushed
            if(_map[newRow][newCol] === Tile.Box || _map[newRow][newCol] === Tile.BoxOnTarget) {
                const newBoxRow = newRow + deltaY;
                const newBoxCol = newCol + deltaX;

                if(_map[newBoxRow][newBoxCol] === Tile.Empty || _map[newBoxRow][newBoxCol] === Tile.Target) {
                    _map[newBoxRow][newBoxCol] = Tile.Box;
                    _map[newRow][newCol] = Tile.Empty;

                    // add animation to the box
                    this.tweens.add({
                        targets: this.boxes.find(b => b.nameCords === `${newRow}-${newCol}`),
                        x: newBoxCol * 64 + 32,
                        y: newBoxRow * 64 + 32,
                        duration: 300, // Animation duration in ms
                        onComplete: () => {
                            let b = this.boxes.find(b => b.nameCords === `${newRow}-${newCol}`);
                            b.nameCords = `${newBoxRow}-${newBoxCol}`;
                        }
                    });
                } else {
                    return;
                }
            }


            this.isMoving = true; // Disable input during movement

            // Update player's internal position
            this.playerRow = newRow;
            this.playerCol = newCol;

            // Tween to smoothly move the player
            this.tweens.add({
                targets: this.player,
                x: this.playerCol * 64 + 32,
                y: this.playerRow * 64 + 32,
                duration: 300, // Animation duration in ms
                onComplete: () => {
                    this.isMoving = false; // Re-enable input when animation ends
                    this.checkWin();
                }
            });

            this.movements.push({
                playerCol: this.playerCol,
                playerRow: this.playerRow,
                playableMap: this.playableMap,
                boxes: this.boxes.map((b, i) => {
                    return {
                        direction: dir,
                        position: i,
                        x: b.x,
                        y: b.y,
                        nameCords: b.nameCords
                    }
                })
            });
        }
    }

    checkWin() {
        // check if the game is win using boxTarget
        if(this.boxes.every(b => {
            return this.boxTarget.includes(b.nameCords);
        })) {
            alert('You win!');
            this.scene.start('Game');
        }
    }
}
