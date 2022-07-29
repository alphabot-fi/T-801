package huobi

import (
	"testing"

	"github.com/alphabot-fi/T-801/internal/proto/exa"
	"github.com/stretchr/testify/assert"
)

func TestString2pair(t *testing.T) {
	p, err := String2pair("ethusdt")
	assert.Nil(t, err)
	assert.Equal(t, exa.Asset_ETH, p.Base)
	assert.Equal(t, exa.Asset_USDT, p.Quote)
	p, err = String2pair("avaxusdt")
	assert.Nil(t, err)
	assert.Equal(t, exa.Asset_AVAX, p.Base)
	assert.Equal(t, exa.Asset_USDT, p.Quote)
	p, err = String2pair("btcusd")
	assert.Nil(t, p)
	assert.EqualError(t, err, "unknown quote asset: 'usd'")
}
