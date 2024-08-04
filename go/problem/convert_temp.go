package problem

func convertTemperature(celsius float64) []float64 {
	// Kelvin = Celsius + 273.15
	// Fahrenheit = Celsius * 1.80 + 32.00
	var result []float64
	result = append(result, celsius+273.15)
	result = append(result, celsius*1.80+32.00)
	return result
}
