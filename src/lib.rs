#[derive(PartialEq, Clone, Debug)]
struct LedColor(i32, i32, i32);

const SENSOR_BATTERY:i32=20;

const OFF:LedColor=LedColor(0,0,0);
const YELLOW:LedColor=LedColor(255,255,0);
const RED:LedColor=LedColor(255,0,0);
const GREEN:LedColor=LedColor(0,255,0);

const PCT_PER_PIXEL:f64=12.5_f64;

extern "C"{
    fn set_led(led_index:i32, red:i32, green:i32, blue:i32);
}

#[no_mangle]
pub extern "C" fn sensor_update(sensor_id:i32, sensor_value:f64)-> f64{
    if sensor_id==SENSOR_BATTERY{
        set_leds(get_led_values(sensor_value));
    }
    sensor_value
}

#[no_mangle]
pub extern "C" fn apply(_frame:u32){
    // do nothing
    //
}

fn get_led_values(battery_remaining:f64) -> [LedColor; 8]{
    let mut arr: [LedColor; 8] = [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF];
    let lit=(battery_remaining/PCT_PER_PIXEL).ceil() as i32;
    let color=if 0.0<=battery_remaining && battery_remaining<=20.0{
       RED 
    }else if 20.0<battery_remaining && battery_remaining<=50.0{
       YELLOW 
    }else{
       GREEN 
    };
    for idx in 0..lit as usize{
        arr[idx]=color.clone();
    }
    arr
}

fn set_leds(leds:[LedColor; 8]){
    for idx in 0..8{
        let LedColor(r,g,b)=leds[idx];
        unsafe{
            set_led(idx as i32, r, g, b);
            
        }
        
    }
    
}




    //use {OFF, YELLOW, RED, GREEN, get_led_values};
#[cfg(test)]
mod tests{
use super::{OFF, YELLOW, RED, GREEN, get_led_values};

    #[test]
    fn test_0_pct(){
        assert_eq!(get_led_values(0.0), [OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF]);
    }
    #[test]
    fn test_15_pct(){
        assert_eq!(get_led_values(15.0), [RED, RED, OFF, OFF, OFF, OFF, OFF, OFF]);
    }
    #[test]
    fn test_49_pct(){
        assert_eq!(get_led_values(49.0), [YELLOW, YELLOW, YELLOW, YELLOW, OFF, OFF, OFF, OFF]);
    }
    #[test]
    fn test_75_pct(){
        assert_eq!(get_led_values(75.0), [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, OFF, OFF]);
    }
    #[test]
    fn test_100_pct(){
        assert_eq!(get_led_values(100.0), [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN]);
    }

}
