

/*
declare class Sniper {
    ['constructor']: typeof Sniper;
    constructor(config?: string | undefined);

    get(attribute: string): string;
}
*/
//let snipe = new Sniper("../snippets");
//export = Sniper;



declare module 'sniper' {
    type add_target=(session_id: string,uri:string , language: string) => void;
    type drop_target=(session_id: string,uri:string , language: string)=> void;
    type get_snippet=(language: string,snippet_key:string)=> Promise<Array<string>>;
    type get_triggers=(session_id: string,uri:string)=> Promise<Array<string>>;
}



