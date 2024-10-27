from dataclasses import dataclass
from typing import List, Dict

# Определим класс "Дом" с количественным признаком, например, "Количество квартир"
@dataclass
class House:
    house_id: int
    address: str
    apartment_count: int
    street_id: int  # Для связи один-ко-многим

# Определим класс "Улица"
@dataclass
class Street:
    street_id: int
    name: str

# Промежуточный класс "Дома на Улице" для связи многие-ко-многим
@dataclass
class HouseOnStreet:
    house_id: int
    street_id: int

# Создание тестовых данных
streets = [
    Street(street_id=1, name="Улица Мира"),
    Street(street_id=2, name="Пролетарская"),
    Street(street_id=3, name="Октябрьская"),
    Street(street_id=4, name="Улица Победы")
]

houses = [
    House(house_id=1, address="Дом 1", apartment_count=10, street_id=1),
    House(house_id=2, address="Дом 2", apartment_count=15, street_id=1),
    House(house_id=3, address="Дом 3", apartment_count=20, street_id=2),
    House(house_id=4, address="Дом 4", apartment_count=25, street_id=2),
    House(house_id=5, address="Дом 5", apartment_count=23, street_id=3),
    House(house_id=6, address="Дом 6", apartment_count=13, street_id=3),
    House(house_id=7, address="Дом 7", apartment_count=15, street_id=3),
    House(house_id=8, address="Дом 8", apartment_count=10, street_id=4),
    House(house_id=9, address="Дом 9", apartment_count=30, street_id=4),
    House(house_id=10, address="Дом 10", apartment_count=20, street_id=4),
    House(house_id=11, address="Дом 11", apartment_count=10, street_id=4)
]

houses_on_streets = [
    HouseOnStreet(house_id=1, street_id=1),
    HouseOnStreet(house_id=2, street_id=1),
    HouseOnStreet(house_id=3, street_id=2),
    HouseOnStreet(house_id=4, street_id=2),
    HouseOnStreet(house_id=5, street_id=3),
    HouseOnStreet(house_id=6, street_id=3),
    HouseOnStreet(house_id=7, street_id=3),
    HouseOnStreet(house_id=8, street_id=4),
    HouseOnStreet(house_id=9, street_id=4),
    HouseOnStreet(house_id=10, street_id=4),
    HouseOnStreet(house_id=11, street_id=4)
]

# Запрос 1: Список всех домов с их квартирами и улицами, отсортированный по улицам
def list_houses_by_street(houses: List[House], streets: List[Street]) -> List[tuple]:
    result = []
    for street in sorted(streets, key=lambda s: s.name):
        for house in filter(lambda h: h.street_id == street.street_id, houses):
            result.append((house.address, house.apartment_count, street.name))
    return result

# Запрос 2: Список улиц с суммарным количеством квартир, отсортированный по количеству квартир
def list_streets_with_total_apartments(houses: List[House], streets: List[Street]) -> List[tuple]:
    street_apartments = {street.street_id: 0 for street in streets}
    for house in houses:
        street_apartments[house.street_id] += house.apartment_count
    sorted_streets = sorted(streets, key=lambda s: street_apartments[s.street_id], reverse=True)
    return [(street.name, street_apartments[street.street_id]) for street in sorted_streets]

# Запрос 3: Список улиц с домами, если в названии улицы есть слово "улиц"
def list_streets_with_houses_with_keyword(streets: List[Street], houses_on_streets: List[HouseOnStreet], keyword: str = "улиц") -> Dict[str, List[str]]:
    result = {}
    for street in filter(lambda s: keyword in s.name.lower(), streets):
        result[street.name] = [
            next(h for h in houses if h.house_id == link.house_id).address
            for link in filter(lambda hs: hs.street_id == street.street_id, houses_on_streets)
        ]
    return result

# Вывод результатов запросов
print("Запрос A1")
print(list_houses_by_street(houses, streets))

print("\nЗапрос A2")
print(list_streets_with_total_apartments(houses, streets))

print("\nЗапрос A3")
print(list_streets_with_houses_with_keyword(streets, houses_on_streets))
