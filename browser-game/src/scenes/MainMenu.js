import { Scene } from 'phaser';

export class MainMenu extends Scene
{
    constructor ()
    {
        super('MainMenu');
    }

    create ()
    {
        //this.add.image(512, 384, 'background');
        
        this.cameras.main.setBackgroundColor(0x648e9a);


        this.add.image(512, 380, 'logo').setScale(0.74);

        this.add.text(512, 685, 'Click to start', {
            fontFamily: 'Arial Black', fontSize: 38, color: '#ffffff',
            stroke: '#000000', strokeThickness: 8,
            align: 'center'
        }).setOrigin(0.5);

        this.input.on('pointerdown', () => {

            const leveldata = window.prompt("Enter the level json data", 
                window.globalLevelData ? JSON.stringify(window.globalLevelData) :
                //'{"row":17,"cols":34,"map":"000000000aaaaaaaa2aaaaaaaa800000022000000008a8a80a2220a2aa8000000008228200088a8aaa02020a2aa88880000002200000000aaaaaa2a20a22aaa000000002200000000aaaaa8a820a200084b2aaa0a8228aa2aa080002a0aa2002202a8880000082a8a2000088aaa828000002aaa8882082a2aaa2200800000002208222a8aaaaaaa0200000000000000aa0","player_row":9,"player_col":17}'
                '{"row":6,"cols":7,"map":"aaaa002844a222bc0aaaa0","player_row":2,"player_col":1}'
            );
            try {
                window.globalLevelData = JSON.parse(leveldata);
                this.scene.start('Game');
            } catch (e) {
                alert("Invalid JSON data");
            }

        });
    }
}
