// THE GAME ITSELF
 
// modules to import
import { GameOptions } from '../GameOptions';
import { LevelThumbnail } from '../LevelThumbnail';
import { PageSelector } from '../PageSelector';
 
const LEVELS = [
    // tutorial
    '{"rows":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/761_microban-1
    '{"rows":7,"cols":7,"map":"aa0238082aad02810a0aaaa000","player_row":3,"player_col":3}',
    //https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/763_microban-3
    '{"rows":6,"cols":9,"map":"0aa02a0aa8004a20928ce0aaaaa0","player_row":4,"player_col":6}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/765_microban-5
    '{"rows":7,"cols":9,"map":"2aaa880022370a84428370a0002aaaa8","player_row":3,"player_col":4}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/766_microban-6
    '{"rows":6,"cols":12,"map":"aaa2aa802a0285002284bf0280aaaaaa8000","player_row":2,"player_col":10}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/767_microban-7
    '{"rows":9,"cols":7,"map":"aaaa0028dca1d28dca1d28dca002aaa8","player_row":7,"player_col":5}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/769_microban-9
    '{"rows":7,"cols":6,"map":"aa8b0a852a022820ae02a0","player_row":2,"player_col":1}',

    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/771_microban-11
    '{"rows":8,"cols":9,"map":"0aaa0200808a2aa2128f84a000282aaaa800","player_row":2,"player_col":6}',
    // https://www.sokobanonline.com/play/web-archive/david-w-skinner/microban/773_microban-13
    '{"rows":9,"cols":7,"map":"aa02ca0b082c60a4a8842200882a2a80","player_row":2,"player_col":2}'
]




// this class extends Scene class
export class LevelSelect extends Phaser.Scene {
 
    // canMove : boolean;
    // itemGroup : Phaser.GameObjects.Group;
    // pageText : Phaser.GameObjects.Text;
    // gameWidth;
    // gameHeight;
    // scrollingMap : Phaser.GameObjects.TileSprite;
    // currentPage;
    // pageSelectors : PageSelector[];
 
    // constructor
    constructor() {
        super({
            key: 'LevelSelect'
        });
    }
 
    preload() {

        this.load.spritesheet("levelthumb", "assets/levelthumb.png", {
            frameWidth : 60,
            frameHeight : 60
        });
 
        this.load.spritesheet('levelpages', 'assets/levelpages.png', {
            frameWidth: 30,
            frameHeight: 30
        });
 
        this.load.image('transp', 'assets/transp.png');
    }

    // method to be called once the class has been created
    create() {
 
        this.initializeProperties();
        this.addBackground();
        this.addInfoText();
        this.addLevelThumbnails();
 
        this.input.setDraggable(this.scrollingMap);
        this.input.on('drag', this.handleDrag, this);
        this.input.on('dragend', this.handleDragEnd, this);
    }    
 
    initializeProperties() {
        this.canMove = true;
        this.gameWidth = this.game.config.width ;
        this.gameHeight = this.game.config.height;
        this.itemGroup = this.add.group();
        this.currentPage = 0;
        this.pageSelectors = [];
    }
 
    addBackground() {   
        this.scrollingMap = this.add.tileSprite(-10, 0, GameOptions.pages * this.gameWidth + 20, this.gameHeight, 'transp');
        this.scrollingMap.setOrigin(0, 0);
        this.scrollingMap.setInteractive();
    }
 
    addInfoText() {
        this.pageText = this.add.text(this.gameWidth / 2, 16, 'Swipe to select level page (1 / ' + GameOptions.pages + ')', {
            font : '18px Arial',
            color : '#ffffff',
            align : 'center'
        });
        this.pageText.setOrigin(0.5);
    }
 
    addLevelThumbnails() {
        let rowLength = GameOptions.thumbWidth * GameOptions.columns + GameOptions.spacing * (GameOptions.columns - 1);
        let leftMargin = (this.gameWidth - rowLength) / 2 + GameOptions.thumbWidth / 2;
        let columnHeight = GameOptions.thumbHeight * GameOptions.rows + GameOptions.spacing * (GameOptions.rows - 1);
        let topMargin = (this.gameHeight - columnHeight) / 2 + GameOptions.thumbHeight / 2;
        for (let k = 0; k < GameOptions.pages; k ++) {
            for (let i = 0; i < GameOptions.columns; i ++) {
                for(let j = 0; j < GameOptions.rows; j ++) {
                    let posX = k * this.gameWidth + leftMargin + i * (GameOptions.thumbWidth + GameOptions.spacing);
                    let posY = topMargin + j * (GameOptions.thumbHeight + GameOptions.spacing);
                    let levelNumber = k * (GameOptions.rows * GameOptions.columns) + j * GameOptions.columns + i;
                    let thumb  = new LevelThumbnail(this, posX, posY, 'levelthumb', levelNumber, levelNumber != 0);
                    thumb.setTint(GameOptions.tintColors[k % GameOptions.tintColors.length]);
                    this.itemGroup.add(thumb);
                    var levelText = this.add.text(thumb.x, thumb.y - 12, thumb.levelNumber.toString(), {
                        font: '24px Arial',
                        color: '#000000'
                    });
                    levelText.setOrigin(0.5);
                    this.itemGroup.add(levelText);

                    thumb.setInteractive();
                    thumb.on('pointerdown', () => {
                        const leveldata = window.prompt("Enter the level json data", 
                            LEVELS[Math.min(thumb.levelNumber, LEVELS.length - 1)]);
                            //window.globalLevelData ? JSON.stringify(window.globalLevelData) :
                            //'{"rows":17,"cols":34,"map":"000000000aaaaaaaa2aaaaaaaa800000022000000008a8a80a2220a2aa8000000008228200088a8aaa02020a2aa88880000002200000000aaaaaa2a20a22aaa000000002200000000aaaaa8a820a200084b2aaa0a8228aa2aa080002a0aa2002202a8880000082a8a2000088aaa828000002aaa8882082a2aaa2200800000002208222a8aaaaaaa0200000000000000aa0","player_row":9,"player_col":17}'
                            //'{"rows":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1}'
                            // );
                        try {
                            window.globalLevelData = JSON.parse(leveldata);
                            this.scene.start('Game');
                        } catch (e) {
                            alert("Invalid JSON data");
                        }

                    })
                }
            }
            this.pageSelectors[k] = new PageSelector(this, this.gameWidth / 2 + (k - Math.floor(GameOptions.pages / 2) + 0.5 * (1 - GameOptions.pages % 2)) * 40, this.gameHeight - 40, 'levelpages', k);
            this.pageSelectors[k].pageIndex = k;
            this.pageSelectors[k].setTint(GameOptions.tintColors[k % GameOptions.tintColors.length])
            if (k == this.currentPage) {
                this.pageSelectors[k].setFrame(1);
            }
            else {
                this.pageSelectors[k].setFrame(0);
            }
        }      
    }
 
    handleDrag(pointer ) {
        if (this.canMove) {
            let deltaX = pointer.position.x - pointer.prevPosition.x;
            if (this.scrollingMap.x + deltaX > 0) {
                deltaX = -this.scrollingMap.x;
            }
            let rightLimit = -((GameOptions.pages - 1) * this.gameWidth + 20)
            if (this.scrollingMap.x + deltaX < rightLimit) {
                deltaX = rightLimit - this.scrollingMap.x; 
            }
            this.scrollingMap.x += deltaX;
            let items  = this.itemGroup.getChildren();
            items.map((item) => {
                item.x += deltaX;
            });   
        }
    }
 
    handleDragEnd(pointer) {
        if (this.canMove) {
            let deltaX = pointer.downX - pointer.position.x;
            if (deltaX == 0) {
                this.canMove = false;
                let items  = this.itemGroup.getChildren() ;
                items.map((item) => {
                    if (item instanceof LevelThumbnail) {
                        let boundingBox = item.getBounds();
                        if (Phaser.Geom.Rectangle.Contains(boundingBox, pointer.position.x, pointer.position.y)) {
                            if (item.locked) {
                                this.tweens.add({
                                    targets : [item],
                                    alpha : 0.2,
                                    duration : 50,
                                    ease : 'Cubic.easeInOut',
                                    yoyo : true,
                                    repeat : 2,
                                    callbackScope: this,
                                    onComplete : this.thumbTweenComplete
                                })
                            }
                        }
                    } 
                }); 
            }
            else {
                if (Math.abs(deltaX) > this.gameWidth / 5) {
                    this.changePage(deltaX > 0 ? 1 : -1);    
                }
                else {
                    this.changePage(0);
                }
            }
        }
    }
 
    thumbTweenComplete(tween , item ) {
        this.canMove = true;
    }
 
    goToPage(page) {
        if (this.canMove) {
            let difference = page - this.currentPage;
            this.changePage(difference);
        }
    }
 
    changePage(amount) {
        this.canMove = false;
        if (this.currentPage + amount < 0 || this.currentPage + amount > GameOptions.pages - 1) {
            amount = 0;
        }
        this.currentPage += amount;
        for (let k = 0; k < GameOptions.pages; k ++) {
            if (k == this.currentPage) {
                this.pageSelectors[k].setFrame(1);
            }
            else {
                this.pageSelectors[k].setFrame(0);
            }
        }
        this.pageText.text = 'Swipe to select level page (' + (this.currentPage + 1).toString() + ' / ' + GameOptions.pages + ')';
        this.tweens.add({
            targets: [this.scrollingMap],
            x: this.currentPage * this.gameWidth * -1 - 10,
            duration: 300,
            ease: 'Cubic.easeOut',
            callbackScope: this,
            onUpdate : this.pageTweenUpdate,
            onComplete : this.pageTweenComplete
        })
    }
 
    pageTweenUpdate(tween, target, position) {
        position = target.x;
        let items  = this.itemGroup.getChildren();
        items.map((item) => {
            let current = tween.data[0].current;
            let previous = tween.data[0].previous;
            item.x += current - previous;
        });
    }
 
    pageTweenComplete() {
        this.canMove = true;
    }
     
     
}