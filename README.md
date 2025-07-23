<div align="center">

# MarketCap
### A fun game to learn about the financial markets

<br><br>
[![Play](https://gist.githubusercontent.com/cxmeel/0dbc95191f239b631c3874f4ccf114e2/raw/play.svg)](https://tvdboom.itch.io/marabunta)
<br><br>
</div>

<img src="https://github.com/tvdboom/marabunta/blob/master/assets/images/scenery/stocks.png?raw=true" alt="Stocks">
<img src="https://github.com/tvdboom/marabunta/blob/master/assets/images/scenery/options.png?raw=true" alt="Options">
<img src="https://github.com/tvdboom/marabunta/blob/master/assets/images/scenery/credit.png?raw=true" alt="Credit">
<img src="https://github.com/tvdboom/marabunta/blob/master/assets/images/scenery/commodities.png?raw=true" alt="Commodities">

<br>

## 📜 Introduction

You are the newly minted CEO of Trident Capital, a scrappy asset manager with ambitions 
to dominate global markets. Backed by a mix of ruthless investors and young in-house talent, 
you're entering the most cutthroat financial landscape seen to date.

Your mission is simple: grow your Assets Under Management (AUM). Trade in stocks, bonds, 
forex, commodities, cryptos and derivatives. Shape politics, rewrite economic policy and 
tilt the balance of power. Navigate corporate scandals, macro shocks and global conflict 
- all while charming clients and outwitting regulators. Your company isn't just about market 
plays - it's a political force.

In this world, success is measured in billions. Are you ready to rewrite history with your 
portfolio? Let the markets open!

<br>

## 🎮 Gameplay

### Economic factors

#### Global economy

The global economy represents the overall financial health and activity of the 
world. It fluctuates based on trade volumes and events. A strong global economy 
means higher consumer confidence, robust industry growth, and increased investments. 
A weak global economy signals recessions, crises, or reduced spending, making 
businesses expansion difficult.

In the game, the global economy (0-100) serves as a macro-scale indicator, affecting 
commodity markets and interest rates dynamically.

#### Inflation

Inflation is the gradual increase in the price of goods and services over time,
reducing the purchasing power of money. As inflation rises, business expenses 
become more expensive.

In the game, there is only one (global) inflation, tied to the global economy, 
where a thriving economy has a higher chance of seeing inflation rise. Inflation 
is also affected by increases in money supply (taking loans) and government policies.

#### Global interest rate

The global interest rate determines the cost of borrowing money. It rises when inflation 
is high, making loans expensive, and falls when inflation is low, encouraging investment.

In the game, interest rates directly impact debt strategies. Players must try to take 
loans during cheap borrowing periods and avoid debt when rates rise. The interest rate 
is updated bi-monthly. At the start of every month, the rate is either updated or the 
next rate is calculated.

<br>

### Player parameters

#### Assets Under Management (AUM)

Your primary goal is to grow your Assets Under Management (AUM). AUM is the total market value
of assets that you manage on behalf of your clients. You lose the game if your AUM drops to zero.

#### Cash

Cash represents the liquid assets the company possesses, funds that are immediately 
available for spending, investing, or covering financial obligations. The bank pays 
a low interest on positive cash deposits and charges a very high interest on negative 
cash deposits.

#### Influence

The influence refers to the capacity or power of the company to affect the behavior, 
and decisions of politicians and other lawmakers in the world. Use this resource to 
lobby politics and policies towards your desired preference. Influence increases over 
time proportionally to your AUM.

<br>

### Trading

You can trade in various asset classes, including stocks, bonds, forex, commodities,
and cryptos. Each asset class has its own market dynamics, risks, and opportunities.

<br>

### Credit

Credit refers to the ability of borrowing money, with the promise of repayment 
in the future. It's a fundamental part of the financial system, allowing companies to 
make purchases, invest, and manage expenses beyond their immediate cash availability.

<br>

### Policies


<br>

### R&D

You can invest in research and development (R&D) to unlock new features and improve your 
trading strategies. There are five fields to research in:

- **Trading**: Unlock new ways to place trading orders.
- **Equities**: Enhance your stocks and bonds capabilities.
- **Alternative investments**: Explore non-traditional asset classes and diversify your portfolio.
- **Credit**: Strengthen your credit possibilities and manage risk effectively.
- **Strategy**: Develop advanced strategies to optimize your decision-making processes.

<br>

### Events

Events are random occurrences that can significantly impact the game. They can be positive 
or negative, affecting prices, exchange rates, interests and the global economy. Events can 
also be triggered by economic conditions, political changes, or other factors.

<br>

## 🗺️ Game schema

See below a schema of how the different game components interact with each other. An arrow
represents a dependency or influence relationship.

<div align="center">
    <img src="https://github.com/tvdboom/marabunta/blob/master/schema.png?raw=true" alt="schema">
</div>

## ⌨️ Key bindings

- `escape`: Enter/exit the in-game menu.
- `space`: Pause/unpause the game.
- `m`: Toggle the audio settings.
