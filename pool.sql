CREATE TABLE pool (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id INTEGER NOT NULL DEFAULT 0,         
    bet_unit INTEGER NOT NULL DEFAULT 0,        
    base_line INTEGER NOT NULL DEFAULT 0,         
    boundary INTEGER NOT NULL DEFAULT 0,       
    brokerage_ratio INTEGER NOT NULL DEFAULT 0,   
    jackpot_ratio INTEGER NOT NULL DEFAULT 0, 
    pot_ratio INTEGER NOT NULL DEFAULT 0,  
    brokerage INTEGER NOT NULL DEFAULT 0,         
    pot INTEGER NOT NULL DEFAULT 0,               
    suction INTEGER NOT NULL DEFAULT 0,           
    jackpot INTEGER NOT NULL DEFAULT 0,                  
    advance INTEGER NOT NULL DEFAULT 0            
);
